#[macro_use] extern crate serde_json;
extern crate splines;

use serde_json::{Value, from_value};
use splines::Spline;

fn main() {
  let value = json!{
    [
      {
        "t": 0,
        "interpolation": "linear",
        "value": 0
      },
      {
        "t": 1,
        "interpolation": { "step": 0.5 },
        "value": 1
      },
      {
        "t": 5,
        "interpolation": "cosine",
        "value": 10
      },
    ]
  };

  let spline = from_value::<Spline<f32>>(value);
  println!("{:?}", spline);
}
