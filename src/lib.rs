//! Spline interpolation made easy.
//!
//! This crate exposes splines for which each sections can be interpolated independently of each
//! other – i.e. it’s possible to interpolate with a linear interpolator on one section and then
//! switch to a cube Hermite interpolatior for the next section.
//!
//! Most of the library consists of three types:
//!
//!   - [`Key`], which represents the control points by which the spline must pass.
//!   - [`Interpolation`], the type of possible interpolation for each segment.
//!   - [`Spline`], a spline from which you can *sample* points by interpolation.
//!
//! When adding control points, you add new sections. Two control points define a section – i.e.
//! it’s not possible to define a spline without at least two control points. Every time you add a
//! new control point, a new section is created. Each section is assigned an interpolation mode that
//! is picked from its lower control point.
//!
//! ```
//! use splines::{Interpolation, Key, Spline};
//!
//! let start = Key::new(0., 0., Interpolation::Linear);
//! let end = Key::new(1., 10., Interpolation::Linear);
//! let spline = Spline::from_keys(vec![start, end]);
//!
//! assert_eq!(spline.sample(0.), Some(0.));
//! assert_eq!(spline.sample(1.), Some(10.));
//! ```

use std::cmp::Ordering;
use std::f32::consts;
use std::ops::{Add, Div, Mul, Sub};

/// A spline control point.
///
/// This type associates a value at a given time. It also contains an interpolation object used to
/// determine how to interpolate values on the segment defined by this key and the next one.
#[derive(Copy, Clone, Debug)]
pub struct Key<T> {
  /// f32 at which the [`Key`] should be reached.
  pub t: f32,
  /// Actual value.
  pub value: T,
  /// Interpolation mode.
  pub interpolation: Interpolation
}

impl<T> Key<T> {
  /// Create a new key.
  pub fn new(t: f32, value: T, interpolation: Interpolation) -> Self {
    Key {
      t: t,
      value: value,
      interpolation: interpolation
    }
  }
}

/// Interpolation mode.
#[derive(Copy, Clone, Debug)]
pub enum Interpolation {
  /// Hold a [`Key`] until the time passes the normalized step threshold, in which case the next
  /// key is used.
  ///
  /// *Note: if you set the threshold to `0.5`, the first key will be used until the time is half
  /// between the two keys; the second key will be in used afterwards. If you set it to `1.0`, the
  /// first key will be kept until the next key. Set it to `0.` and the first key will never be
  /// used.*
  Step(f32),
  /// Linear interpolation between a key and the next one.
  Linear,
  /// Cosine interpolation between a key and the next one.
  Cosine,
  /// Catmull-Rom interpolation.
  CatmullRom
}

impl Default for Interpolation {
  /// `Interpolation::Linear` is the default.
  fn default() -> Self {
    Interpolation::Linear
  }
}

/// Spline curve used to provide interpolation between control points (keys).
#[derive(Debug, Clone)]
pub struct Spline<T>(Vec<Key<T>>);

impl<T> Spline<T> {
  /// Create a new spline out of keys. The keys don’t have to be sorted because they’re sorted by
  /// this function.
  pub fn from_keys(mut keys: Vec<Key<T>>) -> Self {
    keys.sort_by(|k0, k1| k0.t.partial_cmp(&k1.t).unwrap_or(Ordering::Less));

    Spline(keys)
  }

  /// Retrieve the keys of a spline.
  pub fn keys(&self) -> &[Key<T>] {
    &self.0
  }

  /// Sample a spline at a given time.
  ///
  /// # Return
  ///
  /// `None` if you try to sample a value at a time that has no key associated with. That can also
  /// happen if you try to sample between two keys with a specific interpolation mode that make the
  /// sampling impossible. For instance, `Interpolate::CatmullRom` requires *four* keys. If you’re
  /// near the beginning of the spline or its end, ensure you have enough keys around to make the
  /// sampling.
  pub fn sample(&self, t: f32) -> Option<T> where T: Interpolate {
    let first = self.0.first().unwrap();
    let last = self.0.last().unwrap();

    if t <= first.t {
      return Some(first.value);
    } else if t >= last.t {
      return Some(last.value);
    }

    let keys = &self.0;
    let i = keys.binary_search_by(|key| key.t.partial_cmp(&t).unwrap_or(Ordering::Less)).ok()?;

    let cp0 = &keys[i];

    match cp0.interpolation {
      Interpolation::Step(threshold) => {
        let cp1 = &keys[i+1];
        let nt = normalize_time(t, cp0, cp1);
        Some(if nt < threshold { cp0.value } else { cp1.value })
      },
      Interpolation::Linear => {
        let cp1 = &keys[i+1];
        let nt = normalize_time(t, cp0, cp1);

        Some(Interpolate::lerp(cp0.value, cp1.value, nt))
      },
      Interpolation::Cosine => {
        let cp1 = &keys[i+1];
        let nt = normalize_time(t, cp0, cp1);
        let cos_nt = (1. - f32::cos(nt * consts::PI)) * 0.5;

        Some(Interpolate::lerp(cp0.value, cp1.value, cos_nt))
      },
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
}

/// Iterator over spline keys.
pub struct Iter<'a, T> where T: 'a {
  anim_param: &'a Spline<T>,
  i: usize
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a Key<T>;

  fn next(&mut self) -> Option<Self::Item> {
    let r = self.anim_param.0.get(self.i);

    if let Some(_) = r {
      self.i += 1;
    }

    r
  }
}

impl<'a, T> IntoIterator for &'a Spline<T> {
  type Item = &'a Key<T>;
  type IntoIter = Iter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    Iter {
      anim_param: self,
      i: 0
    }
  }
}

/// Keys that can be interpolated in between. Implementing this trait is required to perform
/// sampling on splines.
pub trait Interpolate: Copy {
  /// Linear interpolation.
  fn lerp(a: Self, b: Self, t: f32) -> Self;
  /// Cubic hermite interpolation.
  ///
  /// Default to `Self::lerp`.
  fn cubic_hermite(_: (Self, f32), a: (Self, f32), b: (Self, f32), _: (Self, f32), t: f32) -> Self {
    Self::lerp(a.0, b.0, t)
  }
}

impl Interpolate for f32 {
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    a * (1. - t) + b * t
  }

  fn cubic_hermite(x: (Self, f32), a: (Self, f32), b: (Self, f32), y: (Self, f32), t: f32) -> Self {
    cubic_hermite(x, a, b, y, t)
  }
}

// Default implementation of Interpolate::cubic_hermit.
pub(crate) fn cubic_hermite<T>(x: (T, f32), a: (T, f32), b: (T, f32), y: (T, f32), t: f32) -> T
    where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<f32, Output = T> + Div<f32, Output = T> {
  // time stuff
  let t2 = t * t;
  let t3 = t2 * t;
  let two_t3 = 2. * t3;
  let three_t2 = 3. * t2;

  // tangents
  let m0 = (b.0 - x.0) / (b.1 - x.1);
	let m1 = (y.0 - a.0) / (y.1 - a.1);

  a.0 * (two_t3 - three_t2 + 1.) + m0 * (t3 - 2. * t2 + t) + b.0 * (-two_t3 + three_t2) + m1 * (t3 - t2)
}

// Normalize a time ([0;1]) given two control points.
#[inline(always)]
pub(crate) fn normalize_time<T>(t: f32, cp: &Key<T>, cp1: &Key<T>) -> f32 {
  assert!(cp1.t != cp.t);

  (t - cp.t) / (cp1.t - cp.t)
}
