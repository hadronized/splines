#![cfg(feature = "nalgebra")]

use nalgebra as na;

#[test]
fn nalgebra_vector_interpolation() {
  use splines::Interpolate;

  let start = na::Vector2::new(0.0, 0.0);
  let mid = na::Vector2::new(0.5, 0.5);
  let end = na::Vector2::new(1.0, 1.0);

  assert_eq!(Interpolate::lerp(start, end, 0.0), start);
  assert_eq!(Interpolate::lerp(start, end, 1.0), end);
  assert_eq!(Interpolate::lerp(start, end, 0.5), mid);
}
