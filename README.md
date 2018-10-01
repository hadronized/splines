# splines

This crate provides [splines](https://en.wikipedia.org/wiki/Spline_(mathematics)), mathematic curves
defined piecewise through control keys a.k.a. knots.

Feel free to dig in the [online documentation](https://docs.rs/splines) for further information.

## A note on features

This crate has features! Here’s a comprehensive list of what you can enable:

  - **Serialization / deserialization.**
    + This feature implements both the `Serialize` and `Deserialize` traits from `serde`.
    + Enable with the `"serialization"` feature.
  - **[cgmath](https://crates.io/crates/cgmath) implementors**
    + Adds some usefull implementations of `Interpolate` for some cgmath types.
    + Enable with the `"impl-cgmath"` feature.
  - **Standard library / no standard library.**
    + It’s possible to compile against the standard library or go on your own without it.
    + Compiling with the standard library is enabled by default.
    + Use `default-features = []` in your `Cargo.toml` to disable.
    + Enable explicitly with the `"std"` feature.
