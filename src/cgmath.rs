use cgmath::{BaseNum, InnerSpace, Quaternion, VectorSpace, Vector2, Vector3, Vector4};
use num_traits::Float;

use crate::interpolate::{Interpolate, cubic_hermite_def};

macro_rules! impl_interpolate_vec {
  ($t:ty, $($q:tt)*) => {
    impl Interpolate<$t> for $($q)*<$t> {
      fn lerp(a: Self, b: Self, t: $t) -> Self {
        a.lerp(b, t)
      }

      fn cubic_hermite(x: (Self, $t), a: (Self, $t), b: (Self, $t), y: (Self, $t), t: $t) -> Self {
        cubic_hermite_def(x, a, b, y, t)
      }
    }
  }
}

impl_interpolate_vec!(f32, Vector2);

//impl Interpolate for Quaternion<f32> {
//  fn lerp(a: Self, b: Self, t: f32) -> Self {
//    a.nlerp(b, t)
//  }
//}
