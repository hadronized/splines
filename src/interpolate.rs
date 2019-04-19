#[cfg(feature = "std")] use std::ops::{Div, Mul};
#[cfg(not(feature = "std"))] use core::ops::{Div, Mul};

use num_traits::Float;

/// Keys that can be interpolated in between. Implementing this trait is required to perform
/// sampling on splines.
///
/// `T` is the variable used to sample with. Typical implementations use `f32` or `f64`, but you’re
/// free to use the ones you like.
pub trait Interpolate<T>: Sized + Copy {
  /// Linear interpolation.
  fn lerp(a: Self, b: Self, t: T) -> Self;

  /// Cubic hermite interpolation.
  ///
  /// Default to `Self::lerp`.
  fn cubic_hermite(_: (Self, T), a: (Self, T), b: (Self, T), _: (Self, T), t: T) -> Self {
    Self::lerp(a.0, b.0, t)
  }
}

// Default implementation of Interpolate::cubic_hermite.
//
// `V` is the value being interpolated. `T` is the sampling value (also sometimes called time).
pub(crate) fn cubic_hermite_def<V, T>(x: (V, T), a: (V, T), b: (V, T), y: (V, T), t: T) -> V
where V: Float + Mul<T, Output = V> + Div<T, Output = V>,
      T: Float {
  // some stupid generic constants, because Rust doesn’t have polymorphic literals…
  let two_t = T::one() + T::one(); // lolololol
  let three_t = two_t + T::one(); // megalol

  // sampler stuff
  let t2 = t * t;
  let t3 = t2 * t;
  let two_t3 = t3 * two_t;
  let three_t2 = t2 * three_t;

  // tangents
  let m0 = (b.0 - x.0) / (b.1 - x.1);
  let m1 = (y.0 - a.0) / (y.1 - a.1);

  a.0 * (two_t3 - three_t2 + T::one()) + m0 * (t3 - t2 * two_t + t) + b.0 * (three_t2 - two_t3) + m1 * (t3 - t2)
}

macro_rules! impl_interpolate_simple {
  ($t:ty) => {
    impl Interpolate<$t> for $t {
      fn lerp(a: Self, b: Self, t: $t) -> Self {
        a * (1. - t) + b * t
      }

      fn cubic_hermite(x: (Self, $t), a: (Self, $t), b: (Self, $t), y: (Self, $t), t: $t) -> Self {
        cubic_hermite_def(x, a, b, y, t)
      }
    }
  }
}

impl_interpolate_simple!(f32);
impl_interpolate_simple!(f64);

macro_rules! impl_interpolate_via {
  ($t:ty, $v:ty) => {
    impl Interpolate<$t> for $v {
      fn lerp(a: Self, b: Self, t: $t) -> Self {
        a * (1. - t as $v) + b * t as $v
      }

      fn cubic_hermite((x, xt): (Self, $t), (a, at): (Self, $t), (b, bt): (Self, $t), (y, yt): (Self, $t), t: $t) -> Self {
        cubic_hermite_def((x, xt as $v), (a, at as $v), (b, bt as $v), (y, yt as $v), t as $v)
      }
    }
  }
}

impl_interpolate_via!(f32, f64);
impl_interpolate_via!(f64, f32);

