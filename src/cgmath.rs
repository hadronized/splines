use cgmath::{
  BaseFloat, BaseNum, InnerSpace, Quaternion, VectorSpace, Vector1, Vector2, Vector3, Vector4
};

use crate::interpolate::{Additive, Interpolate, Linear, One, cubic_hermite_def};

macro_rules! impl_interpolate_vec {
  ($($t:tt)*) => {
    impl<T> Linear<T> for $($t)*<T> where T: BaseNum {
      fn outer_mul(self, t: T) -> Self {
        self * t
      }

      fn outer_div(self, t: T) -> Self {
        self / t
      }
    }

    impl<T> Interpolate<T> for $($t)*<T>
    where Self: InnerSpace<Scalar = T>, T: Additive + BaseFloat + One {
      fn lerp(a: Self, b: Self, t: T) -> Self {
        a.lerp(b, t)
      }

      fn cubic_hermite(x: (Self, T), a: (Self, T), b: (Self, T), y: (Self, T), t: T) -> Self {
        cubic_hermite_def(x, a, b, y, t)
      }
    }
  }
}

impl_interpolate_vec!(Vector1);
impl_interpolate_vec!(Vector2);
impl_interpolate_vec!(Vector3);
impl_interpolate_vec!(Vector4);

impl<T> Linear<T> for Quaternion<T> where T: BaseFloat {
  fn outer_mul(self, t: T) -> Self {
    self * t
  }

  fn outer_div(self, t: T) -> Self {
    self / t
  }
}

impl<T> Interpolate<T> for Quaternion<T>
where Self: InnerSpace<Scalar = T>, T: Additive + BaseFloat + One {
  fn lerp(a: Self, b: Self, t: T) -> Self {
    a.nlerp(b, t)
  }

  fn cubic_hermite(x: (Self, T), a: (Self, T), b: (Self, T), y: (Self, T), t: T) -> Self {
    cubic_hermite_def(x, a, b, y, t)
  }
}
