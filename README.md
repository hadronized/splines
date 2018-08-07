# splines

This crate provides [splines](https://en.wikipedia.org/wiki/Spline_(mathematics)), mathematic curves
defined piecewise through control keys a.k.a. knots.

Feel free to dig in the [online documentation](https://docs.rs/splines) for further information.

## A note on features

This crate has features! Here’s a comprehensive list of what you can enable:

  - **Serialization / deserialization**
    + This feature implements both the `Serialize` and `Deserialize` traits from `serde`.
    + Enable with the feature `"serialization"`.
