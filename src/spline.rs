#[cfg(feature = "serialization")] use serde_derive::{Deserialize, Serialize};
#[cfg(not(feature = "std"))] use alloc::vec::Vec;
#[cfg(feature = "std")] use std::cmp::Ordering;
#[cfg(feature = "std")] use std::ops::{Div, Mul};
#[cfg(not(feature = "std"))] use core::ops::{Div, Mul};
#[cfg(not(feature = "std"))] use core::cmp::Ordering;

use crate::interpolate::{Interpolate, Additive, One, Trigo};
use crate::interpolation::Interpolation;
use crate::key::Key;

/// Spline curve used to provide interpolation between control points (keys).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Spline<T, V>(pub(crate) Vec<Key<T, V>>);

impl<T, V> Spline<T, V> {
  /// Create a new spline out of keys. The keys don’t have to be sorted even though it’s recommended
  /// to provide ascending sorted ones (for performance purposes).
  pub fn from_vec(mut keys: Vec<Key<T, V>>) -> Self where T: PartialOrd {
    keys.sort_by(|k0, k1| k0.t.partial_cmp(&k1.t).unwrap_or(Ordering::Less));

    Spline(keys)
  }

  /// Create a new spline by consuming an `Iterater<Item = Key<T>>`. They keys don’t have to be
  /// sorted.
  ///
  /// # Note on iterators
  ///
  /// It’s valid to use any iterator that implements `Iterator<Item = Key<T>>`. However, you should
  /// use `Spline::from_vec` if you are passing a `Vec<_>`. This will remove dynamic allocations.
  pub fn from_iter<I>(iter: I) -> Self where I: Iterator<Item = Key<T, V>>, T: PartialOrd {
    Self::from_vec(iter.collect())
  }

  /// Retrieve the keys of a spline.
  pub fn keys(&self) -> &[Key<T, V>] {
    &self.0
  }

  /// Sample a spline at a given time.
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
  /// sampling impossible. For instance, `Interpolate::CatmullRom` requires *four* keys. If you’re
  /// near the beginning of the spline or its end, ensure you have enough keys around to make the
  /// sampling.
  pub fn sample(&self, t: T) -> Option<V> where T: Additive + One + Trigo + Mul<T, Output = T> + Div<T, Output = T>, V: Interpolate<T> {
    let keys = &self.0;
    let i = search_lower_cp(keys, t)?;
    let cp0 = &keys[i];

    match cp0.interpolation {
      Interpolation::Step(threshold) => {
        let cp1 = &keys[i+1];
        let nt = normalize_time(t, cp0, cp1);
        Some(if nt < threshold { cp0.value } else { cp1.value })
      }

      Interpolation::Linear => {
        let cp1 = &keys[i+1];
        let nt = normalize_time(t, cp0, cp1);

        Some(Interpolate::lerp(cp0.value, cp1.value, nt))
      }

      Interpolation::Cosine => {
        let two_t = T::one() + T::one();
        let cp1 = &keys[i+1];
        let nt = normalize_time(t, cp0, cp1);
        let cos_nt = (T::one() - (nt * T::pi()).cos()) / two_t;

        Some(Interpolate::lerp(cp0.value, cp1.value, cos_nt))
      }

      Interpolation::CatmullRom => {
        // We need at least four points for Catmull Rom; ensure we have them, otherwise, return
        // None.
        if i == 0 || i >= keys.len() - 2 {
          None
        } else {
          let cp1 = &keys[i+1];
          let cpm0 = &keys[i-1];
          let cpm1 = &keys[i+2];
          let nt = normalize_time(t, cp0, cp1);

          Some(Interpolate::cubic_hermite((cpm0.value, cpm0.t), (cp0.value, cp0.t), (cp1.value, cp1.t), (cpm1.value, cpm1.t), nt))
        }
      }
    }
  }

  /// Sample a spline at a given time with clamping.
  ///
  /// # Return
  ///
  /// If you sample before the first key or after the last one, return the first key or the last
  /// one, respectively. Otherwise, behave the same way as `Spline::sample`.
  ///
  /// # Error
  ///
  /// This function returns `None` if you have no key.
  pub fn clamped_sample(&self, t: T) -> Option<V> where T: Additive + One + Trigo + Mul<T, Output = T> + Div<T, Output = T>, V: Interpolate<T> {
    if self.0.is_empty() {
      return None;
    }

    self.sample(t).or_else(move || {
      let first = self.0.first().unwrap();
      if t <= first.t {
        Some(first.value)
      } else {
        let last = self.0.last().unwrap();

        if t >= last.t {
          Some(last.value)
        } else {
          None
        }
      }
    })
  }
}

// Normalize a time ([0;1]) given two control points.
#[inline(always)]
pub(crate) fn normalize_time<T, V>(
  t: T,
  cp: &Key<T, V>,
  cp1: &Key<T, V>
) -> T where T: Additive + Div<T, Output = T> {
  assert!(cp1.t != cp.t, "overlapping keys");
  (t - cp.t) / (cp1.t - cp.t)
}

// Find the lower control point corresponding to a given time.
fn search_lower_cp<T, V>(cps: &[Key<T, V>], t: T) -> Option<usize> where T: PartialOrd {
  let mut i = 0;
  let len = cps.len();

  if len < 2 {
    return None;
  }

  loop {
    let cp = &cps[i];
    let cp1 = &cps[i+1];

    if t >= cp1.t {
      if i >= len - 2 {
        return None;
      }

      i += 1;
    } else if t < cp.t {
      if i == 0 {
        return None;
      }

      i -= 1;
    } else {
      break; // found
    }
  }

  Some(i)
}
