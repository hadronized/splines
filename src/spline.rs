//! Spline curves and operations.

#[cfg(feature = "std")]
use crate::interpolate::{Interpolate, Interpolator};
use crate::interpolation::Interpolation;
use crate::key::Key;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
use core::cmp::Ordering;
#[cfg(not(feature = "std"))]
use core::ops::{Div, Mul};
#[cfg(any(feature = "serialization", feature = "serde"))]
use serde::{Deserialize, Serialize};
#[cfg(feature = "std")]
use std::cmp::Ordering;

/// Spline curve used to provide interpolation between control points (keys).
///
/// Splines are made out of control points ([`Key`]). When creating a [`Spline`] with
/// [`Spline::from_vec`] or [`Spline::from_iter`], the keys don’t have to be sorted (they are sorted
/// automatically by the sampling value).
///
/// You can sample from a spline with several functions:
///
///   - [`Spline::sample`]: allows you to sample from a spline. If not enough keys are available
///     for the required interpolation mode, you get `None`.
///   - [`Spline::clamped_sample`]: behaves like [`Spline::sample`] but will return either the first
///     or last key if out of bound; it will return `None` if not enough key.
#[derive(Debug, Clone, Default)]
#[cfg_attr(
  any(feature = "serialization", feature = "serde"),
  derive(Deserialize, Serialize)
)]
pub struct Spline<T, V>(pub(crate) Vec<Key<T, V>>);

impl<T, V> Spline<T, V> {
  /// Internal sort to ensure invariant of sorting keys is valid.
  fn internal_sort(&mut self)
  where
    T: PartialOrd,
  {
    self
      .0
      .sort_by(|k0, k1| k0.t.partial_cmp(&k1.t).unwrap_or(Ordering::Less));
  }

  /// Create a new spline out of keys. The keys don’t have to be sorted even though it’s recommended
  /// to provide ascending sorted ones (for performance purposes).
  pub fn from_vec(keys: Vec<Key<T, V>>) -> Self
  where
    T: PartialOrd,
  {
    let mut spline = Spline(keys);
    spline.internal_sort();
    spline
  }

  /// Clear the spline by removing all keys. Keeps the underlying allocated storage, so adding
  /// new keys should be faster than creating a new [`Spline`]
  #[inline]
  pub fn clear(&mut self) {
    self.0.clear()
  }

  /// Create a new spline by consuming an `Iterater<Item = Key<T>>`. They keys don’t have to be
  /// sorted.
  ///
  /// # Note on iterators
  ///
  /// It’s valid to use any iterator that implements `Iterator<Item = Key<T>>`. However, you should
  /// use [`Spline::from_vec`] if you are passing a [`Vec`].
  pub fn from_iter<I>(iter: I) -> Self
  where
    I: Iterator<Item = Key<T, V>>,
    T: PartialOrd,
  {
    Self::from_vec(iter.collect())
  }

  /// Retrieve the keys of a spline.
  pub fn keys(&self) -> &[Key<T, V>] {
    &self.0
  }

  /// Number of keys.
  #[inline(always)]
  pub fn len(&self) -> usize {
    self.0.len()
  }

  /// Check whether the spline has no key.
  #[inline(always)]
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  /// Sample a spline at a given time, returning the interpolated value along with its associated
  /// key.
  ///
  /// The current implementation, based on immutability, cannot perform in constant time. This means
  /// that sampling’s processing complexity is currently *O(log n)*. It’s possible to achieve *O(1)*
  /// performance by using a slightly different spline type. If you are interested by this feature,
  /// an implementation for a dedicated type is foreseen yet not started yet.
  ///
  /// # Return
  ///
  /// `None` if you try to sample a value at a time that has no key associated with. That can also
  /// happen if you try to sample between two keys with a specific interpolation mode that makes the
  /// sampling impossible. For instance, [`Interpolation::CatmullRom`] requires *four* keys. If
  /// you’re near the beginning of the spline or its end, ensure you have enough keys around to make
  /// the sampling.
  pub fn sample_with_key(&self, t: T) -> Option<SampledWithKey<V>>
  where
    T: Interpolator,
    V: Interpolate<T>,
  {
    let keys = &self.0;
    let i = search_lower_cp(keys, t)?;
    let cp0 = &keys[i];

    let value = match cp0.interpolation {
      Interpolation::Step(threshold) => {
        let cp1 = &keys[i + 1];
        let nt = t.normalize(cp0.t, cp1.t);
        let value = V::step(nt, threshold, cp0.value, cp1.value);

        Some(value)
      }

      Interpolation::Linear => {
        let cp1 = &keys[i + 1];
        let nt = t.normalize(cp0.t, cp1.t);
        let value = V::lerp(nt, cp0.value, cp1.value);

        Some(value)
      }

      Interpolation::Cosine => {
        let cp1 = &keys[i + 1];
        let nt = t.normalize(cp0.t, cp1.t);
        let value = V::cosine(nt, cp0.value, cp1.value);

        Some(value)
      }

      Interpolation::CatmullRom => {
        // We need at least four points for Catmull Rom; ensure we have them, otherwise, return
        // None.
        if i == 0 || i >= keys.len() - 2 {
          None
        } else {
          let cp1 = &keys[i + 1];
          let cpm0 = &keys[i - 1];
          let cpm1 = &keys[i + 2];
          let nt = t.normalize(cp0.t, cp1.t);
          let value = V::cubic_hermite(
            nt,
            (cpm0.t, cpm0.value),
            (cp0.t, cp0.value),
            (cp1.t, cp1.value),
            (cpm1.t, cpm1.value),
          );

          Some(value)
        }
      }

      Interpolation::Bezier(u) | Interpolation::StrokeBezier(_, u) => {
        // We need to check the next control point to see whether we want quadratic or cubic Bezier.
        let cp1 = &keys[i + 1];
        let nt = t.normalize(cp0.t, cp1.t);

        let value = match cp1.interpolation {
          Interpolation::Bezier(v) => V::cubic_bezier_mirrored(nt, cp0.value, u, v, cp1.value),

          Interpolation::StrokeBezier(v, _) => V::cubic_bezier(nt, cp0.value, u, v, cp1.value),

          _ => V::quadratic_bezier(nt, cp0.value, u, cp1.value),
        };

        Some(value)
      }
    };

    value.map(|value| SampledWithKey { value, key: i })
  }

  /// Sample a spline at a given time.
  ///
  pub fn sample(&self, t: T) -> Option<V>
  where
    T: Interpolator,
    V: Interpolate<T>,
  {
    self.sample_with_key(t).map(|sampled| sampled.value)
  }

  /// Sample a spline at a given time with clamping, returning the interpolated value along with its
  /// associated key.
  ///
  /// # Return
  ///
  /// If you sample before the first key or after the last one, return the first key or the last
  /// one, respectively. Otherwise, behave the same way as [`Spline::sample`].
  ///
  /// # Error
  ///
  /// This function returns [`None`] if you have no key.
  pub fn clamped_sample_with_key(&self, t: T) -> Option<SampledWithKey<V>>
  where
    T: Interpolator,
    V: Interpolate<T>,
  {
    if self.0.is_empty() {
      return None;
    }

    self.sample_with_key(t).or_else(move || {
      let first = self.0.first().unwrap();

      if t <= first.t {
        let sampled = SampledWithKey {
          value: first.value,
          key: 0,
        };
        Some(sampled)
      } else {
        let last = self.0.last().unwrap();

        if t >= last.t {
          let sampled = SampledWithKey {
            value: last.value,
            key: self.0.len() - 1,
          };
          Some(sampled)
        } else {
          None
        }
      }
    })
  }

  /// Sample a spline at a given time with clamping.
  pub fn clamped_sample(&self, t: T) -> Option<V>
  where
    T: Interpolator,
    V: Interpolate<T>,
  {
    self.clamped_sample_with_key(t).map(|sampled| sampled.value)
  }

  /// Add a key into the spline.
  pub fn add(&mut self, key: Key<T, V>)
  where
    T: PartialOrd,
  {
    self.0.push(key);
    self.internal_sort();
  }

  /// Remove a key from the spline.
  pub fn remove(&mut self, index: usize) -> Option<Key<T, V>> {
    if index >= self.0.len() {
      None
    } else {
      Some(self.0.remove(index))
    }
  }

  /// Update a key and return the key already present.
  ///
  /// The key is updated — if present — with the provided function.
  ///
  /// # Notes
  ///
  /// That function makes sense only if you want to change the interpolator (i.e. [`Key::t`]) of
  /// your key. If you just want to change the interpolation mode or the carried value, consider
  /// using the [`Spline::get_mut`] method instead as it will be way faster.
  pub fn replace<F>(&mut self, index: usize, f: F) -> Option<Key<T, V>>
  where
    F: FnOnce(&Key<T, V>) -> Key<T, V>,
    T: PartialOrd,
  {
    let key = self.remove(index)?;
    self.add(f(&key));
    Some(key)
  }

  /// Get a key at a given index.
  pub fn get(&self, index: usize) -> Option<&Key<T, V>> {
    self.0.get(index)
  }

  /// Mutably get a key at a given index.
  pub fn get_mut(&mut self, index: usize) -> Option<KeyMut<T, V>> {
    self.0.get_mut(index).map(|key| KeyMut {
      value: &mut key.value,
      interpolation: &mut key.interpolation,
    })
  }
}

/// A sampled value along with its key index.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SampledWithKey<V> {
  /// Sampled value.
  pub value: V,

  /// Key index.
  pub key: usize,
}

/// A mutable [`Key`].
///
/// Mutable keys allow to edit the carried values and the interpolation mode but not the actual
/// interpolator value as it would invalidate the internal structure of the [`Spline`]. If you
/// want to achieve this, you’re advised to use [`Spline::replace`].
#[derive(Debug)]
pub struct KeyMut<'a, T, V> {
  /// Carried value.
  pub value: &'a mut V,
  /// Interpolation mode to use for that key.
  pub interpolation: &'a mut Interpolation<T, V>,
}

// Find the lower control point corresponding to a given time.
// It has the property to have a timestamp smaller or equal to t
fn search_lower_cp<T, V>(cps: &[Key<T, V>], t: T) -> Option<usize>
where
  T: PartialOrd,
{
  let len = cps.len();
  if len < 2 {
    return None;
  }
  match cps.binary_search_by(|key| key.t.partial_cmp(&t).unwrap()) {
    Err(i) if i >= len => None,
    Err(i) if i == 0 => None,
    Err(i) => Some(i - 1),
    Ok(i) if i == len - 1 => None,
    Ok(i) => Some(i),
  }
}
