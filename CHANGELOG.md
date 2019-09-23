# 1.1

> Mon Sep 22rd 2019

- Yanked.

# 1.0

> Sun Sep 22nd 2019

## Major changes

- Make `Spline::clamped_sample` failible via `Option` instead of panicking.
- Add support for polymorphic sampling type.

## Minor changes

- Add the `std` feature (and hence support for `no_std`).
- Add `impl-nalgebra` feature.
- Add `impl-cgmath` feature.
- Add support for adding keys to splines.
- Add support for removing keys from splines.

## Patch changes

- Migrate to Rust 2018.
- Documentation typo fixes.

# 0.2.3

> Sat 13th October 2018

- Add the `"impl-nalgebra"` feature gate. It gives access to some implementors for the `nalgebra`
  crate.
- Enhance the documentation.

# 0.2.2

> Sun 30th September 2018

- Bump version numbers (`splines-0.2`) in examples.
- Fix several typos in the documentation.

# 0.2.1

> Thu 20th September 2018

- Enhance the features documentation.

# 0.2

> Thu 6th September 2018

- Add the `"std"` feature gate, that can be used to compile with the standard library.
- Add the `"impl-cgmath"` feature gate in order to make optional, if wanted, the `cgmath`
  dependency.
- Enhance the documentation.

# 0.1.1

> Wed 8th August 2018

- Add a feature gate, `"serialization"`, that can be used to automatically derive `Serialize` and
  `Deserialize` from the [serde](https://crates.io/crates/serde) crate.
- Enhance the documentation.

# 0.1

> Sunday 5th August 2018

- Initial revision.
