//! The [`Interpolate`] trait and associated symbols.
//!
//! The [`Interpolate`] trait is the central concept of the crate. It enables a spline to be
//! sampled at by interpolating in between control points.
//!
//! In order for a type to be used in [`Spline<K, V>`], some properties must be met about the `K`
//! type must implementing several traits:
//!
//!   - [`One`], giving a neutral element for the multiplication monoid.
//!   - [`Additive`], making the type additive (i.e. one can add or subtract with it).
//!   - [`Linear`], unlocking linear combinations, required for interpolating.
//!   - [`Trigo`], a trait giving *π* and *cosine*, required for e.g. cosine interpolation.
//!
//! Feel free to have a look at current implementors for further help.
//!
//! > *Why doesn’t this crate use [num-traits] instead of
//! > defining its own traits?*
//!
//! The reason for this is quite simple: this crate provides a `no_std` support, which is not
//! currently available easily with [num-traits]. Also, if something changes in [num-traits] with
//! those traits, it would make this whole crate unstable.
//!
//! [`Interpolate`]: crate::interpolate::Interpolate
//! [`Spline<K, V>`]: crate::spline::Spline
//! [`One`]: crate::interpolate::One
//! [`Additive`]: crate::interpolate::Additive
//! [`Linear`]: crate::interpolate::Linear
//! [`Trigo`]: crate::interpolate::Trigo
//! [num-traits]: https://crates.io/crates/num-traits

#[cfg(not(feature = "std"))]
use core::f32;
#[cfg(not(feature = "std"))]
use core::f64;
#[cfg(not(feature = "std"))]
use core::intrinsics::cosf32;
#[cfg(not(feature = "std"))]
use core::intrinsics::cosf64;
#[cfg(not(feature = "std"))]
use core::ops::{Add, Mul, Sub};
#[cfg(feature = "std")]
use std::f32;
#[cfg(feature = "std")]
use std::f64;

/// Types that can be used as interpolator in splines.
///
/// An interpolator value is like the fabric on which control keys (and sampled values) live on.
pub trait Interpolator: Sized + Copy + PartialOrd {
  /// Normalize the interpolator.
  fn normalize(self, start: Self, end: Self) -> Self;
}

macro_rules! impl_Interpolator {
  ($t:ty) => {
    impl Interpolator for $t {
      fn normalize(self, start: Self, end: Self) -> Self {
        (self - start) / (end - start)
      }
    }
  };
}

impl_Interpolator!(f32);
impl_Interpolator!(f64);

/// Values that can be interpolated. Implementing this trait is required to perform sampling on splines.
///
/// `T` is the interpolator used to sample with. Typical implementations use [`f32`] or [`f64`], but
/// you’re free to use the ones you like.
pub trait Interpolate<T>: Sized + Copy {
  /// Step interpolation.
  fn step(t: T, threshold: T, a: Self, b: Self) -> Self;

  /// Linear interpolation.
  fn lerp(t: T, a: Self, b: Self) -> Self;

  /// Cosine interpolation.
  fn cosine(t: T, a: Self, b: Self) -> Self;

  /// Cubic hermite interpolation.
  fn cubic_hermite(t: T, x: (T, Self), a: (T, Self), b: (T, Self), y: (T, Self)) -> Self;

  /// Quadratic Bézier interpolation.
  ///
  /// `a` is the first point; `b` is the second point and `u` is the tangent of `a` to the curve.
  fn quadratic_bezier(t: T, a: Self, u: Self, b: Self) -> Self;

  /// Cubic Bézier interpolation.
  ///
  /// `a` is the first point; `b` is the second point; `u` is the output tangent of `a` to the curve and `v` is the
  /// input tangent of `b` to the curve.
  fn cubic_bezier(t: T, a: Self, u: Self, v: Self, b: Self) -> Self;

  /// Cubic Bézier interpolation – special case for non-explicit second tangent.
  ///
  /// This version does the same computation as [`Interpolate::cubic_bezier`] but computes the second tangent by
  /// inversing it (typical when the next point uses a Bézier interpolation, where input and output tangents are
  /// mirrored for the same key).
  fn cubic_bezier_mirrored(t: T, a: Self, u: Self, v: Self, b: Self) -> Self;
}

#[macro_export]
macro_rules! impl_Interpolate {
  ($t:ty, $v:ty, $pi:expr) => {
    impl $crate::interpolate::Interpolate<$t> for $v {
      fn step(t: $t, threshold: $t, a: Self, b: Self) -> Self {
        if t < threshold {
          a
        } else {
          b
        }
      }

      fn cosine(t: $t, a: Self, b: Self) -> Self {
        let cos_nt = (1. - (t * $pi).cos()) * 0.5;
        <Self as $crate::interpolate::Interpolate<$t>>::lerp(cos_nt, a, b)
      }

      fn lerp(t: $t, a: Self, b: Self) -> Self {
        a * (1. - t) + b * t
      }

      fn cubic_hermite(t: $t, x: ($t, Self), a: ($t, Self), b: ($t, Self), y: ($t, Self)) -> Self {
        // sampler stuff
        let two_t = t * 2.;
        let three_t = t * 3.;
        let t2 = t * t;
        let t3 = t2 * t;
        let two_t3 = t2 * two_t;
        let two_t2 = t * two_t;
        let three_t2 = t * three_t;

        // tangents
        let m0 = (b.1 - x.1) / (b.0 - x.0);
        let m1 = (y.1 - a.1) / (y.0 - a.0);

        a.1 * (two_t3 - three_t2 + 1.)
          + m0 * (t3 - two_t2 + t)
          + b.1 * (three_t2 - two_t3)
          + m1 * (t3 - t2)
      }

      fn quadratic_bezier(t: $t, a: Self, u: Self, b: Self) -> Self {
        let one_t = 1. - t;
        let one_t2 = one_t * one_t;

        u + (a - u) * one_t2 + (b - u) * t * t
      }

      fn cubic_bezier(t: $t, a: Self, u: Self, v: Self, b: Self) -> Self {
        let one_t = 1. - t;
        let one_t2 = one_t * one_t;
        let one_t3 = one_t2 * one_t;
        let t2 = t * t;

        a * one_t3 + (u * one_t2 * t + v * one_t * t2) * 3. + b * t2 * t
      }

      fn cubic_bezier_mirrored(t: $t, a: Self, u: Self, v: Self, b: Self) -> Self {
        <Self as $crate::interpolate::Interpolate<$t>>::cubic_bezier(t, a, u, b + b - v, b)
      }
    }
  };
}

#[macro_export]
macro_rules! impl_InterpolateT {
  ($t:ty, $v:ty, $pi:expr) => {
    impl $crate::interpolate::Interpolate<$t> for $v {
      fn step(t: $t, threshold: $t, a: Self, b: Self) -> Self {
        if t < threshold {
          a
        } else {
          b
        }
      }

      fn cosine(t: $t, a: Self, b: Self) -> Self {
        let cos_nt = (1. - (t * $pi).cos()) * 0.5;
        <Self as $crate::interpolate::Interpolate<$t>>::lerp(cos_nt, a, b)
      }

      fn lerp(t: $t, a: Self, b: Self) -> Self {
        let t = Self::from(t);
        a * (1. - t) + b * t
      }

      fn cubic_hermite(t: $t, x: ($t, Self), a: ($t, Self), b: ($t, Self), y: ($t, Self)) -> Self {
        // sampler stuff
        let t = Self::from(t);
        let two_t = t * 2.;
        let three_t = t * 3.;
        let t2 = t * t;
        let t3 = t2 * t;
        let two_t3 = t2 * two_t;
        let two_t2 = t * two_t;
        let three_t2 = t * three_t;

        // tangents
        let m0 = (b.1 - x.1) / (Self::from(b.0 - x.0));
        let m1 = (y.1 - a.1) / (Self::from(y.0 - a.0));

        a.1 * (two_t3 - three_t2 + 1.)
          + m0 * (t3 - two_t2 + t)
          + b.1 * (three_t2 - two_t3)
          + m1 * (t3 - t2)
      }

      fn quadratic_bezier(t: $t, a: Self, u: Self, b: Self) -> Self {
        let t = Self::from(t);
        let one_t = 1. - t;
        let one_t2 = one_t * one_t;

        u + (a - u) * one_t2 + (b - u) * t * t
      }

      fn cubic_bezier(t: $t, a: Self, u: Self, v: Self, b: Self) -> Self {
        let t = Self::from(t);
        let one_t = 1. - t;
        let one_t2 = one_t * one_t;
        let one_t3 = one_t2 * one_t;
        let t2 = t * t;

        a * one_t3 + (u * one_t2 * t + v * one_t * t2) * 3. + b * t2 * t
      }

      fn cubic_bezier_mirrored(t: $t, a: Self, u: Self, v: Self, b: Self) -> Self {
        <Self as $crate::interpolate::Interpolate<$t>>::cubic_bezier(t, a, u, b + b - v, b)
      }
    }
  };
}

impl_Interpolate!(f32, f32, std::f32::consts::PI);
impl_Interpolate!(f64, f64, std::f64::consts::PI);
impl_InterpolateT!(f32, f64, std::f32::consts::PI);
