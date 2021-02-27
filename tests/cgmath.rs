#![cfg(feature = "cgmath")]

use cgmath as cg;
use splines::{Interpolation, Key, Spline};

#[test]
fn cgmath_vector_interpolation() {
  use splines::Interpolate;

  let start = cg::Vector2::new(0.0, 0.0);
  let mid = cg::Vector2::new(0.5, 0.5);
  let end = cg::Vector2::new(1.0, 1.0);

  assert_eq!(Interpolate::lerp(start, end, 0.0), start);
  assert_eq!(Interpolate::lerp(start, end, 1.0), end);
  assert_eq!(Interpolate::lerp(start, end, 0.5), mid);
}

#[test]
fn stroke_bezier_straight() {
  use float_cmp::approx_eq;

  let keys = vec![
    Key::new(
      0.0,
      cg::Vector2::new(0., 1.),
      Interpolation::StrokeBezier(cg::Vector2::new(0., 1.), cg::Vector2::new(0., 1.)),
    ),
    Key::new(
      5.0,
      cg::Vector2::new(5., 1.),
      Interpolation::StrokeBezier(cg::Vector2::new(5., 1.), cg::Vector2::new(5., 1.)),
    ),
  ];
  let spline = Spline::from_vec(keys);

  assert!(approx_eq!(f32, spline.clamped_sample(0.0).unwrap().y, 1.));
  assert!(approx_eq!(f32, spline.clamped_sample(1.0).unwrap().y, 1.));
  assert!(approx_eq!(f32, spline.clamped_sample(2.0).unwrap().y, 1.));
  assert!(approx_eq!(f32, spline.clamped_sample(3.0).unwrap().y, 1.));
  assert!(approx_eq!(f32, spline.clamped_sample(4.0).unwrap().y, 1.));
  assert!(approx_eq!(f32, spline.clamped_sample(5.0).unwrap().y, 1.));
}
