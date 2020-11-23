use glam::{Quat, Vec2, Vec3, Vec3A, Vec4};

use crate::interpolate::{
  cubic_bezier_def, cubic_hermite_def, quadratic_bezier_def, Interpolate, Linear,
};

macro_rules! impl_interpolate_vec {
  ($($t:tt)*) => {
    impl Linear<f32> for $($t)* {
      #[inline(always)]
      fn outer_mul(self, t: f32) -> Self {
        self * t
      }

      #[inline(always)]
      fn outer_div(self, t: f32) -> Self {
        self / t
      }
    }

    impl Interpolate<f32> for $($t)* {
      #[inline(always)]
      fn lerp(a: Self, b: Self, t: f32) -> Self {
        a.lerp(b, t)
      }

      #[inline(always)]
      fn cubic_hermite(
        x: (Self, f32),
        a: (Self, f32),
        b: (Self, f32),
        y: (Self, f32),
        t: f32,
      ) -> Self {
        cubic_hermite_def(x, a, b, y, t)
      }

      #[inline(always)]
      fn quadratic_bezier(a: Self, u: Self, b: Self, t: f32) -> Self {
        quadratic_bezier_def(a, u, b, t)
      }

      #[inline(always)]
      fn cubic_bezier(a: Self, u: Self, v: Self, b: Self, t: f32) -> Self {
        cubic_bezier_def(a, u, v, b, t)
      }
    }
  }
}

impl_interpolate_vec!(Vec2);
impl_interpolate_vec!(Vec3);
impl_interpolate_vec!(Vec3A);
impl_interpolate_vec!(Vec4);

impl Linear<f32> for Quat {
  #[inline(always)]
  fn outer_mul(self, t: f32) -> Self {
    self * t
  }

  #[inline(always)]
  fn outer_div(self, t: f32) -> Self {
    self / t
  }
}

impl Interpolate<f32> for Quat {
  #[inline(always)]
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    a.lerp(b, t)
  }

  #[inline(always)]
  fn cubic_hermite(x: (Self, f32), a: (Self, f32), b: (Self, f32), y: (Self, f32), t: f32) -> Self {
    cubic_hermite_def(x, a, b, y, t)
  }

  #[inline(always)]
  fn quadratic_bezier(a: Self, u: Self, b: Self, t: f32) -> Self {
    quadratic_bezier_def(a, u, b, t)
  }

  #[inline(always)]
  fn cubic_bezier(a: Self, u: Self, v: Self, b: Self, t: f32) -> Self {
    cubic_bezier_def(a, u, v, b, t)
  }
}
