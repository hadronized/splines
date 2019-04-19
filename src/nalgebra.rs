use crate::Interpolate;

 use nalgebra as na;

use num_traits::Float;

macro_rules! impl_interpolate_na_vector {
  ($($t:tt)*) => {
    impl<T, V> Interpolate<T> for $($t)*<V> where T: Float, V: na::Scalar + Interpolate<T> {
      fn lerp(a: Self, b: Self, t: T) -> Self {
        na::Vector::zip_map(&a, &b, |c1, c2| Interpolate::lerp(c1, c2, t))
      }
    }
  }
}

impl_interpolate_na_vector!(na::Vector1);
impl_interpolate_na_vector!(na::Vector2);
impl_interpolate_na_vector!(na::Vector3);
impl_interpolate_na_vector!(na::Vector4);
impl_interpolate_na_vector!(na::Vector5);
impl_interpolate_na_vector!(na::Vector6);

impl<T, N, D> Interpolate<T> for na::Point<N, D>
where D: na::DimName,
      na::DefaultAllocator: na::allocator::Allocator<N, D>,
      <na::DefaultAllocator as na::allocator::Allocator<N, D>>::Buffer: Copy,
      N: na::Scalar + Interpolate<T>,
      T: Float {
  fn lerp(a: Self, b: Self, t: T) -> Self {
    // The 'coords' of a point is just a vector, so we can interpolate component-wise
    // over these vectors.
    let coords = na::Vector::zip_map(&a.coords, &b.coords, |c1, c2| Interpolate::lerp(c1, c2, t));
    na::Point::from(coords)
  }
}
