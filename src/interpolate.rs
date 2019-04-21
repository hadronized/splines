#[cfg(feature = "std")] use std::f32;
#[cfg(not(feature = "std"))] use core::f32;
#[cfg(not(feature = "std"))] use core::intrinsics::cosf32;
#[cfg(feature = "std")] use std::f64;
#[cfg(not(feature = "std"))] use core::f64;
#[cfg(not(feature = "std"))] use core::intrinsics::cosf64;
#[cfg(feature = "std")] use std::ops::{Add, Mul, Sub};
#[cfg(not(feature = "std"))] use core::ops::{Add, Mul, Sub};

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

/// A trait for anything that supports additions, subtraction, multiplication and division.
pub trait Additive:
  Copy +
  Add<Self, Output = Self> +
  Sub<Self, Output = Self> {
}

impl<T> Additive for T
where T: Copy +
         Add<Self, Output = Self> +
         Sub<Self, Output = Self> {
}

/// Linear combination.
pub trait Linear<T> {
  /// Apply an outer multiplication law.
  fn outer_mul(self, t: T) -> Self;

  /// Apply an outer division law.
  fn outer_div(self, t: T) -> Self;
}

macro_rules! impl_linear_simple {
  ($t:ty) => {
    impl Linear<$t> for $t {
      fn outer_mul(self, t: $t) -> Self {
        self * t
      }

      /// Apply an outer division law.
      fn outer_div(self, t: $t) -> Self {
        self / t
      }
    }
  }
}

impl_linear_simple!(f32);
impl_linear_simple!(f64);

macro_rules! impl_linear_cast {
  ($t:ty, $q:ty) => {
    impl Linear<$t> for $q {
      fn outer_mul(self, t: $t) -> Self {
        self * t as $q
      }

      /// Apply an outer division law.
      fn outer_div(self, t: $t) -> Self {
        self / t as $q
      }
    }
  }
}

impl_linear_cast!(f32, f64);
impl_linear_cast!(f64, f32);

/// Types with a neutral element for multiplication.
pub trait One {
  /// Return the neutral element for the multiplicative monoid.
  fn one() -> Self;
}

macro_rules! impl_one_float {
  ($t:ty) => {
    impl One for $t {
      #[inline(always)]
      fn one() -> Self {
        1.
      }
    }
  }
}

impl_one_float!(f32);
impl_one_float!(f64);

/// Types with a sane definition of π and cosine.
pub trait Trigo {
  /// π.
  fn pi() -> Self;

  /// Cosine of the argument.
  fn cos(self) -> Self;
}

impl Trigo for f32 {
  #[inline(always)]
  fn pi() -> Self {
    f32::consts::PI
  }

  #[inline(always)]
  fn cos(self) -> Self {
    #[cfg(feature = "std")]
    {
      self.cos()
    }

    #[cfg(not(feature = "std"))]
    {
      unsafe { cosf32(self) }
    }
  }
}

impl Trigo for f64 {
  #[inline(always)]
  fn pi() -> Self {
    f64::consts::PI
  }

  #[inline(always)]
  fn cos(self) -> Self {
    #[cfg(feature = "std")]
    {
      self.cos()
    }

    #[cfg(not(feature = "std"))]
    {
      unsafe { cosf64(self) }
    }
  }
}

// Default implementation of Interpolate::cubic_hermite.
//
// `V` is the value being interpolated. `T` is the sampling value (also sometimes called time).
pub(crate) fn cubic_hermite_def<V, T>(x: (V, T), a: (V, T), b: (V, T), y: (V, T), t: T) -> V
where V: Additive + Linear<T>,
      T: Additive + Mul<T, Output = T> + One {
  // some stupid generic constants, because Rust doesn’t have polymorphic literals…
  let one_t = T::one();
  let two_t = one_t + one_t; // lolololol
  let three_t = two_t + one_t; // megalol

  // sampler stuff
  let t2 = t * t;
  let t3 = t2 * t;
  let two_t3 = t3 * two_t;
  let three_t2 = t2 * three_t;

  // tangents
  let m0 = (b.0 - x.0).outer_div(b.1 - x.1);
  let m1 = (y.0 - a.0).outer_div(y.1 - a.1);

  a.0.outer_mul(two_t3 - three_t2 + one_t) + m0.outer_mul(t3 - t2 * two_t + t) + b.0.outer_mul(three_t2 - two_t3) + m1.outer_mul(t3 - t2)
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
