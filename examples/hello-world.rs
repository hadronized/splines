extern crate splines;

use splines::{Interpolation, Key, Spline};

fn main() {
  let keys = vec![Key::new(0., 0., Interpolation::default()), Key::new(5., 1., Interpolation::default())];
  let spline = Spline::from_vec(keys);

  println!("value at 0: {:?}", spline.clamped_sample(0.));
  println!("value at 3: {:?}", spline.clamped_sample(3.));
}
