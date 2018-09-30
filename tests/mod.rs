extern crate splines;
#[cfg(feature = "impl-nalgebra")] extern crate nalgebra;

use splines::{Interpolation, Key, Spline, Interpolate};
#[cfg(feature = "impl-nalgebra")] use nalgebra as na;


#[test]
fn step_interpolation_0() {
  let start  = Key::new(0., 0., Interpolation::Step(0.));
  let end    = Key::new(1., 10., Interpolation::default());
  let spline = Spline::from_vec(vec![start, end]);

  assert_eq!(spline.sample(0.), Some(10.));
  assert_eq!(spline.sample(0.1), Some(10.));
  assert_eq!(spline.sample(0.2), Some(10.));
  assert_eq!(spline.sample(0.5), Some(10.));
  assert_eq!(spline.sample(0.9), Some(10.));
  assert_eq!(spline.sample(1.), None);
  assert_eq!(spline.clamped_sample(1.), 10.);
}

#[test]
fn step_interpolation_0_5() {
  let start  = Key::new(0., 0., Interpolation::Step(0.5));
  let end    = Key::new(1., 10., Interpolation::default());
  let spline = Spline::from_vec(vec![start, end]);

  assert_eq!(spline.sample(0.), Some(0.));
  assert_eq!(spline.sample(0.1), Some(0.));
  assert_eq!(spline.sample(0.2), Some(0.));
  assert_eq!(spline.sample(0.5), Some(10.));
  assert_eq!(spline.sample(0.9), Some(10.));
  assert_eq!(spline.sample(1.), None);
  assert_eq!(spline.clamped_sample(1.), 10.);
}

#[test]
fn step_interpolation_0_75() {
  let start  = Key::new(0., 0., Interpolation::Step(0.75));
  let end    = Key::new(1., 10., Interpolation::default());
  let spline = Spline::from_vec(vec![start, end]);

  assert_eq!(spline.sample(0.), Some(0.));
  assert_eq!(spline.sample(0.1), Some(0.));
  assert_eq!(spline.sample(0.2), Some(0.));
  assert_eq!(spline.sample(0.5), Some(0.));
  assert_eq!(spline.sample(0.9), Some(10.));
  assert_eq!(spline.sample(1.), None);
  assert_eq!(spline.clamped_sample(1.), 10.);
}

#[test]
fn step_interpolation_1() {
  let start  = Key::new(0., 0., Interpolation::Step(1.));
  let end    = Key::new(1., 10., Interpolation::default());
  let spline = Spline::from_vec(vec![start, end]);

  assert_eq!(spline.sample(0.), Some(0.));
  assert_eq!(spline.sample(0.1), Some(0.));
  assert_eq!(spline.sample(0.2), Some(0.));
  assert_eq!(spline.sample(0.5), Some(0.));
  assert_eq!(spline.sample(0.9), Some(0.));
  assert_eq!(spline.sample(1.), None);
  assert_eq!(spline.clamped_sample(1.), 10.);
}

#[test]
fn linear_interpolation() {
  let start  = Key::new(0., 0., Interpolation::Linear);
  let end    = Key::new(1., 10., Interpolation::default());
  let spline = Spline::from_vec(vec![start, end]);

  assert_eq!(spline.sample(0.), Some(0.));
  assert_eq!(spline.sample(0.1), Some(1.));
  assert_eq!(spline.sample(0.2), Some(2.));
  assert_eq!(spline.sample(0.5), Some(5.));
  assert_eq!(spline.sample(0.9), Some(9.));
  assert_eq!(spline.sample(1.), None);
  assert_eq!(spline.clamped_sample(1.), 10.);
}

#[test]
fn linear_interpolation_several_keys() {
  let start = Key::new(0., 0., Interpolation::Linear);
  let k1    = Key::new(1., 5., Interpolation::Linear);
  let k2    = Key::new(2., 0., Interpolation::Linear);
  let k3    = Key::new(3., 1., Interpolation::Linear);
  let k4    = Key::new(10., 2., Interpolation::Linear);
  let end   = Key::new(11., 4., Interpolation::default());
  let spline = Spline::from_vec(vec![start, k1, k2, k3, k4, end]);

  assert_eq!(spline.sample(0.), Some(0.));
  assert_eq!(spline.sample(0.1), Some(0.5));
  assert_eq!(spline.sample(0.2), Some(1.));
  assert_eq!(spline.sample(0.5), Some(2.5));
  assert_eq!(spline.sample(0.9), Some(4.5));
  assert_eq!(spline.sample(1.), Some(5.));
  assert_eq!(spline.sample(1.5), Some(2.5));
  assert_eq!(spline.sample(2.), Some(0.));
  assert_eq!(spline.sample(2.75), Some(0.75));
  assert_eq!(spline.sample(3.), Some(1.));
  assert_eq!(spline.sample(6.5), Some(1.5));
  assert_eq!(spline.sample(10.), Some(2.));
  assert_eq!(spline.clamped_sample(11.), 4.);
}

#[test]
fn several_interpolations_several_keys() {
  let start = Key::new(0., 0., Interpolation::Step(0.5));
  let k1    = Key::new(1., 5., Interpolation::Linear);
  let k2    = Key::new(2., 0., Interpolation::Step(0.1));
  let k3    = Key::new(3., 1., Interpolation::Linear);
  let k4    = Key::new(10., 2., Interpolation::Linear);
  let end   = Key::new(11., 4., Interpolation::default());
  let spline = Spline::from_vec(vec![start, k1, k2, k3, k4, end]);

  assert_eq!(spline.sample(0.), Some(0.));
  assert_eq!(spline.sample(0.1), Some(0.));
  assert_eq!(spline.sample(0.2), Some(0.));
  assert_eq!(spline.sample(0.5), Some(5.));
  assert_eq!(spline.sample(0.9), Some(5.));
  assert_eq!(spline.sample(1.), Some(5.));
  assert_eq!(spline.sample(1.5), Some(2.5));
  assert_eq!(spline.sample(2.), Some(0.));
  assert_eq!(spline.sample(2.05), Some(0.));
  assert_eq!(spline.sample(2.1), Some(0.));
  assert_eq!(spline.sample(2.75), Some(1.));
  assert_eq!(spline.sample(3.), Some(1.));
  assert_eq!(spline.sample(6.5), Some(1.5));
  assert_eq!(spline.sample(10.), Some(2.));
  assert_eq!(spline.clamped_sample(11.), 4.);
}

#[test]
#[cfg(feature = "impl-nalgebra")]
fn nalgebra_point_interpolation() {
    let start : na::Point2<f32> = na::Point2::new(0.0, 0.0);
    let mid   : na::Point2<f32> = na::Point2::new(0.5, 0.5);
    let end   : na::Point2<f32> = na::Point2::new(1.0, 1.0);

    assert_eq!(Interpolate::lerp(start, end, 0.0), start);
    assert_eq!(Interpolate::lerp(start, end, 1.0), end);
    assert_eq!(Interpolate::lerp(start, end, 0.5), mid);
}

#[test]
#[cfg(feature = "impl-nalgebra")]
fn nalgebra_vector_interpolation() {
    let start : na::Vector2<f32> = na::Vector2::new(0.0, 0.0);
    let mid   : na::Vector2<f32> = na::Vector2::new(0.5, 0.5);
    let end   : na::Vector2<f32> = na::Vector2::new(1.0, 1.0);

    assert_eq!(Interpolate::lerp(start, end, 0.0), start);
    assert_eq!(Interpolate::lerp(start, end, 1.0), end);
    assert_eq!(Interpolate::lerp(start, end, 0.5), mid);
}

