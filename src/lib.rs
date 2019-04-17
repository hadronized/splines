//! # Spline interpolation made easy.
//!
//! This crate exposes splines for which each sections can be interpolated independently of each
//! other – i.e. it’s possible to interpolate with a linear interpolator on one section and then
//! switch to a cubic Hermite interpolator for the next section.
//!
//! Most of the crate consists of three types:
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
//! # Quickly create splines
//!
//! ```
//! use splines::{Interpolation, Key, Spline};
//!
//! let start = Key::new(0., 0., Interpolation::Linear);
//! let end = Key::new(1., 10., Interpolation::default());
//! let spline = Spline::from_vec(vec![start, end]);
//! ```
//!
//! You will notice that we used `Interpolation::Linear` for the first key. The first key `start`’s
//! interpolation will be used for the whole segment defined by those two keys. The `end`’s
//! interpolation won’t be used. You can in theory use any [`Interpolation`] you want for the last
//! key. We use the default one because we don’t care.
//!
//! # Interpolate values
//!
//! The whole purpose of splines is to interpolate discrete values to yield continuous ones. This is
//! usually done with the `Spline::sample` method. This method expects the interpolation parameter
//! (often, this will be the time of your simulation) as argument and will yield an interpolated
//! value.
//!
//! If you try to sample in out-of-bounds interpolation parameter, you’ll get no value.
//!
//! ```
//! # use splines::{Interpolation, Key, Spline};
//! # let start = Key::new(0., 0., Interpolation::Linear);
//! # let end = Key::new(1., 10., Interpolation::Linear);
//! # let spline = Spline::from_vec(vec![start, end]);
//! assert_eq!(spline.sample(0.), Some(0.));
//! assert_eq!(spline.clamped_sample(1.), Some(10.));
//! assert_eq!(spline.sample(1.1), None);
//! ```
//!
//! It’s possible that you want to get a value even if you’re out-of-bounds. This is especially
//! important for simulations / animations. Feel free to use the `Spline::clamped_interpolation` for
//! that purpose.
//!
//! ```
//! # use splines::{Interpolation, Key, Spline};
//! # let start = Key::new(0., 0., Interpolation::Linear);
//! # let end = Key::new(1., 10., Interpolation::Linear);
//! # let spline = Spline::from_vec(vec![start, end]);
//! assert_eq!(spline.clamped_sample(-0.9), Some(0.)); // clamped to the first key
//! assert_eq!(spline.clamped_sample(1.1), Some(10.)); // clamped to the last key
//! ```
//!
//! # Features and customization
//!
//! This crate was written with features baked in and hidden behind feature-gates. The idea is that
//! the default configuration (i.e. you just add `"splines = …"` to your `Cargo.toml`) will always
//! give you the minimal, core and raw concepts of what splines, keys / knots and interpolation
//! modes are. However, you might want more. Instead of letting other people do the extra work to
//! add implementations for very famous and useful traits – and do it in less efficient way, because
//! they wouldn’t have access to the internals of this crate, it’s possible to enable features in an
//! ad hoc way.
//!
//! This mechanism is not final and this is currently an experiment to see how people like it or
//! not. It’s especially important to see how it copes with the documentation.
//!
//! So here’s a list of currently supported features and how to enable them:
//!
//!   - **Serialization / deserialization.**
//!     + This feature implements both the `Serialize` and `Deserialize` traits from `serde` for all
//!       types exported by this crate.
//!     + Enable with the `"serialization"` feature.
//!   - **[cgmath](https://crates.io/crates/cgmath) implementors.**
//!     + Adds some useful implementations of `Interpolate` for some cgmath types.
//!     + Enable with the `"impl-cgmath"` feature.
//!   - **[nalgebra](https://crates.io/crates/nalgebra) implementors.**
//!     + Adds some useful implementations of `Interpolate` for some nalgebra types.
//!     + Enable with the `"impl-nalgebra"` feature.
//!   - **Standard library / no standard library.**
//!     + It’s possible to compile against the standard library or go on your own without it.
//!     + Compiling with the standard library is enabled by default.
//!     + Use `default-features = []` in your `Cargo.toml` to disable.
//!     + Enable explicitly with the `"std"` feature.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]
#![cfg_attr(not(feature = "std"), feature(core_intrinsics))]

// on no_std, we also need the alloc crate for Vec
#[cfg(not(feature = "std"))] extern crate alloc;

#[cfg(feature = "impl-cgmath")] extern crate cgmath;

#[cfg(feature = "impl-nalgebra")] extern crate nalgebra;

#[cfg(feature = "serialization")] extern crate serde;
#[cfg(feature = "serialization")] #[macro_use] extern crate serde_derive;

#[cfg(feature = "impl-cgmath")]
use cgmath::{
  BaseFloat, InnerSpace, Quaternion, Vector2, Vector3, Vector4
};

#[cfg(feature = "impl-nalgebra")] use nalgebra as na;
#[cfg(feature = "impl-nalgebra")] use nalgebra::core::{DimName, DefaultAllocator, Scalar};
#[cfg(feature = "impl-nalgebra")] use nalgebra::core::allocator::Allocator;

#[cfg(feature = "std")] use std::cmp::Ordering;
#[cfg(feature = "std")] use std::ops::{Add, Div, Mul, Sub};

#[cfg(not(feature = "std"))] use alloc::vec::Vec;
#[cfg(not(feature = "std"))] use core::cmp::Ordering;
#[cfg(not(feature = "std"))] use core::ops::{Add, Div, Mul, Sub};

use num_traits::{Float, FloatConst};

/// A spline control point.
///
/// This type associates a value at a given interpolation parameter value. It also contains an
/// interpolation hint used to determine how to interpolate values on the segment defined by this
/// key and the next one – if existing.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serialization", serde(rename_all = "snake_case"))]
pub struct Key<T, V> {
  /// Interpolation parameter at which the [`Key`] should be reached.
  pub t: T,
  /// Held value.
  pub value: V,
  /// Interpolation mode.
  pub interpolation: Interpolation<T>
}

impl<T, V> Key<T, V> {
  /// Create a new key.
  pub fn new(t: T, value: V, interpolation: Interpolation<T>) -> Self {
    Key { t, value, interpolation }
  }
}

/// Interpolation mode.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serialization", serde(rename_all = "snake_case"))]
pub enum Interpolation<T> {
  /// Hold a [`Key`] until the interpolator value passes the normalized step threshold, in which
  /// case the next key is used.
  ///
  /// > Note: if you set the threshold to `0.5`, the first key will be used until half the time
  /// > between the two keys; the second key will be in used afterwards. If you set it to `1.0`, the
  /// > first key will be kept until the next key. Set it to `0.` and the first key will never be
  /// > used.
  Step(T),
  /// Linear interpolation between a key and the next one.
  Linear,
  /// Cosine interpolation between a key and the next one.
  Cosine,
  /// Catmull-Rom interpolation, performing a cubic Hermite interpolation using four keys.
  CatmullRom
}

impl<T> Default for Interpolation<T> {
  /// `Interpolation::Linear` is the default.
  fn default() -> Self {
    Interpolation::Linear
  }
}

/// Spline curve used to provide interpolation between control points (keys).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
pub struct Spline<T, V>(Vec<Key<T, V>>);

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
  /// happen if you try to sample between two keys with a specific interpolation mode that make the
  /// sampling impossible. For instance, `Interpolate::CatmullRom` requires *four* keys. If you’re
  /// near the beginning of the spline or its end, ensure you have enough keys around to make the
  /// sampling.
  pub fn sample(&self, t: T) -> Option<V> where V: Interpolate<T>, T: Float {
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
        let cp1 = &keys[i+1];
        let nt = normalize_time(t, cp0, cp1);
        let cos_nt = (1. - (nt * T::PI).cos()) * 0.5;

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
  pub fn clamped_sample(&self, t: f32) -> Option<T> where T: Interpolate {
    if self.0.is_empty() {
      return None;
    }

    let first = self.0.first().unwrap();
    let last = self.0.last().unwrap();

    let sampled = if t <= first.t {
      first.value
    } else if t >= last.t {
      last.value
    } else {
      self.sample(t).unwrap()
    };

    Some(sampled)
  }
}

/// Iterator over spline keys.
///
/// This iterator type assures you to iterate over sorted keys.
pub struct Iter<'a, T, V> where T: 'a, V: 'a {
  anim_param: &'a Spline<T, V>,
  i: usize
}

impl<'a, T, V> Iterator for Iter<'a, T, V> {
  type Item = &'a Key<T, V>;

  fn next(&mut self) -> Option<Self::Item> {
    let r = self.anim_param.0.get(self.i);

    if let Some(_) = r {
      self.i += 1;
    }

    r
  }
}

impl<'a, T, V> IntoIterator for &'a Spline<T, V> {
  type Item = &'a Key<T, V>;
  type IntoIter = Iter<'a, T, V>;

  fn into_iter(self) -> Self::IntoIter {
    Iter {
      anim_param: self,
      i: 0
    }
  }
}

/// Keys that can be interpolated in between. Implementing this trait is required to perform
/// sampling on splines.
///
/// `T` is the variable used to sample with. Typical implementations use `f32` or `f64`, but you’re
/// free to use the ones you like.
pub trait Interpolate<T>: Copy where T: Copy + Float {
  /// Linear interpolation.
  fn lerp(a: Self, b: Self, t: T) -> Self;

  /// Cubic hermite interpolation.
  ///
  /// Default to `Self::lerp`.
  fn cubic_hermite(_: (Self, T), a: (Self, T), b: (Self, T), _: (Self, T), t: T) -> Self {
    Self::lerp(a.0, b.0, t)
  }
}

impl Interpolate<f32> for f32 {
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    a * (1. - t) + b * t
  }

  fn cubic_hermite(x: (Self, f32), a: (Self, f32), b: (Self, f32), y: (Self, f32), t: f32) -> Self {
    cubic_hermite(x, a, b, y, t)
  }
}

impl Interpolate<f32> for f64 {
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    a * (1. - t as f64) + b * t as f64
  }

  fn cubic_hermite(
    (x, tx): (Self, f32),
    (a, ta): (Self, f32),
    (b, tb): (Self, f32),
    (y, ty): (Self, f32),
    t: f32
  ) -> Self {
    cubic_hermite((x, tx as f64), (a, ta as f64), (b, tb as f64), (y, ty as f64), t as f64)
  }
}

#[cfg(feature = "impl-cgmath")]
impl<T> Interpolate<T> for Vector2<T> where T: BaseFloat {
  fn lerp(a: Self, b: Self, t: T) -> Self {
    a.lerp(b, t)
  }

  fn cubic_hermite(x: (Self, T), a: (Self, T), b: (Self, T), y: (Self, T), t: T) -> Self {
    cubic_hermite(x, a, b, y, t)
  }
}

#[cfg(feature = "impl-cgmath")]
impl<T> Interpolate<T> for Vector3<T> where T: BaseFloat {
  fn lerp(a: Self, b: Self, t: T) -> Self {
    a.lerp(b, t)
  }

  fn cubic_hermite(x: (Self, T), a: (Self, T), b: (Self, T), y: (Self, T), t: T) -> Self {
    cubic_hermite(x, a, b, y, t)
  }
}

#[cfg(feature = "impl-cgmath")]
impl<T> Interpolate<T> for Vector4<T> where T: BaseFloat {
  fn lerp(a: Self, b: Self, t: T) -> Self {
    a.lerp(b, t)
  }

  fn cubic_hermite(x: (Self, T), a: (Self, T), b: (Self, T), y: (Self, T), t: T) -> Self {
    cubic_hermite(x, a, b, y, t)
  }
}

#[cfg(feature = "impl-cgmath")]
impl<T> Interpolate<T> for Quaternion<T> where T: BaseFloat {
  fn lerp(a: Self, b: Self, t: T) -> Self {
    a.nlerp(b, t)
  }
}

#[cfg(feature = "impl-nalgebra")]
impl<N, D> Interpolate for na::Point<N, D>
where D: DimName,
      DefaultAllocator: Allocator<N, D>,
      <DefaultAllocator as Allocator<N, D>>::Buffer: Copy,
      N: Scalar + Interpolate {
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    // The 'coords' of a point is just a vector, so we can interpolate component-wise
    // over these vectors.
    let coords = na::Vector::zip_map(&a.coords, &b.coords, |c1, c2| Interpolate::lerp(c1, c2, t));
    na::Point::from_coordinates(coords)
  }
}

#[cfg(feature = "impl-nalgebra")]
impl Interpolate for na::Vector1<f32> {
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    na::Vector::zip_map(&a, &b, |c1, c2| Interpolate::lerp(c1, c2, t))
  }
}

#[cfg(feature = "impl-nalgebra")]
impl Interpolate for na::Vector2<f32> {
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    na::Vector::zip_map(&a, &b, |c1, c2| Interpolate::lerp(c1, c2, t))
  }
}

#[cfg(feature = "impl-nalgebra")]
impl Interpolate for na::Vector3<f32> {
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    na::Vector::zip_map(&a, &b, |c1, c2| Interpolate::lerp(c1, c2, t))
  }
}

#[cfg(feature = "impl-nalgebra")]
impl Interpolate for na::Vector4<f32> {
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    na::Vector::zip_map(&a, &b, |c1, c2| Interpolate::lerp(c1, c2, t))
  }
}

#[cfg(feature = "impl-nalgebra")]
impl Interpolate for na::Vector5<f32> {
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    na::Vector::zip_map(&a, &b, |c1, c2| Interpolate::lerp(c1, c2, t))
  }
}

#[cfg(feature = "impl-nalgebra")]
impl Interpolate for na::Vector6<f32> {
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    na::Vector::zip_map(&a, &b, |c1, c2| Interpolate::lerp(c1, c2, t))
  }
}

// Default implementation of Interpolate::cubic_hermite.
//
// `V` is the value being interpolated. `T` is the sampling value (also sometimes called time).
pub(crate) fn cubic_hermite<V, T>(x: (V, T), a: (V, T), b: (V, T), y: (V, T), t: T) -> V
where V: Copy + Add<Output = V> + Sub<Output = V> + Mul<T, Output = V> + Div<T, Output = V>,
      T: Mul<Output = T> + Mul<f32, Output = T> + Add<Output = T> + Add<f32, Output = T> + Sub<Output = T> {
  // sampler stuff
  let t2 = t* t;
  let t3 = t2 * t;
  let two_t3 = t3 * 2.;
  let three_t2 = t2 * 3.;

  // tangents
  let m0 = (b.0 - x.0) / (b.1 - x.1);
	let m1 = (y.0 - a.0) / (y.1 - a.1);

  a.0 * (two_t3 - three_t2 + 1.) + m0 * (t3 - t2 * 2. + t) + b.0 * (three_t2 - two_t3) + m1 * (t3 - t2)
}

// Normalize a time ([0;1]) given two control points.
#[inline(always)]
pub(crate) fn normalize_time<T, V>(
  t: T,
  cp: &Key<T, V>,
  cp1: &Key<T, V>
) -> T where T: PartialEq + Sub<Output = T> + Div<Output = T> {
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
