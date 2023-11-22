# Changelog

* [4.3.1](#431)
* [4.3](#43)
* [4.2](#42)
* [4.1.1](#411)
* [4.1](#41)
* [4.0.3](#403)
* [4.0.2](#402)
* [4.0.1](#401)
* [4.0](#40)
  * [Major changes](#major-changes)
  * [Patch changes](#patch-changes)
* [3.5.4](#354)
* [3.5.3](#353)
* [3.5.2](#352)
* [3.5.1](#351)
* [3.5](#35)
* [3.4.2](#342)
* [3.4.1](#341)
* [3.4](#34)
* [3.3](#33)
* [3.2](#32)
* [3.1](#31)
* [3.0](#30)
  * [Major changes](#major-changes-1)
  * [Patch changes](#patch-changes-1)
* [2.2](#22)
* [2.1.1](#211)
* [2.1](#21)
* [2.0.1](#201)
* [2.0](#20)
  * [Major changes](#major-changes-2)
  * [Minor changes](#minor-changes)
* [1.0](#10)
  * [Major changes](#major-changes-3)
  * [Minor changes](#minor-changes-1)
  * [Patch changes](#patch-changes-2)
* [0.2.3](#023)
* [0.2.2](#022)
* [0.2.1](#021)
* [0.2](#02)
* [0.1.1](#011)
* [0.1](#01)

# 4.3.1

> Nov 22, 2023

- Add `Default` implementation for `Spline`. [c6ba847](https://github.com/phaazon/splines/commit/c6ba847)

# 4.3

> Sep 23, 2023

- Add support for `glam-0.23` and `glam-0.24`. [cdc48a4](https://github.com/phaazon/splines/commit/cdc48a4)
- Add `Spline::clear` to clear a spline keys without deallocating its internal storage. [eca09f1](https://github.com/phaazon/splines/commit/eca09f1)

# 4.2

> Feb 1, 2023

- Add support for `glam-0.22`.
- Add support for `nalgebra-0.32`.
- Add deprecation lints for `impl-*` feature gates. Those shouldn’t be used anymore and the `*` variant should be 
  preferred. For instance, if you used `impl-cgmath`, you should just use the `cgmath` feature gate now.

# 4.1.1

> Jul 27, 2022

- Internal enhancement of sampling splines by looking for control points. That brings the lookup from _O(N)_ to
  _O(log(N))_. That is super embarassing because it should have been the default from the very first commit. Sorry
  about that.
- Fix hermite cubic interpolation.
- Add support for `glam-0.21`.
- Add support for `nalgebra-0.31`.

# 4.1

> Mar 28, 2022

- Support for edition 2021.
- Bump `float-cmp` dependency.
- Bump `glam` dependency.
- Bump `nalgebra` dependency.
- Simplify the CI.

# 4.0.3

> Jul 11, 2021

- Add more implementors for `Interpolate`.

# 4.0.2

> Jul 11, 2021

- **Yanked.**

# 4.0.1

> Jul 11, 2021

- Add support up to `glam-0.17`.
- Add support up to `nalgebra-0.27`.
- Replace the name of some feature gates:
  - `serialization` becomes `serde`.
  - `impl-*` becomes `*`.
  - The previous feature gates are kept around to prevent a breaking change but will eventually be removed in the next
    major update.

# 4.0

> Mar 05, 2021

## Major changes

- Switch the `Interpolation` enum to `#[non_exhaustive]` to allow adding more interpolation modes (if any) in the
  future.
- Introduce `SampledWithKey`, which is a more elegant / typed way to access a sample along with its associated key
  index.
- Refactor the `Interpolate` trait and add the `Interpolator` trait.

## Patch changes

- Highly simplify the various implementors (`cgmath`, `nalgebra` and `glam`) so that maintenance is easy.
- Expose the `impl_Interpolate` macro, allowing to implement the API all at once if a type implements the various
  `std::ops:*` traits. Since most of the crates do, this macro makes it really easy to add support for a crate.
- Drop `simba` as a direct dependency.
- Drop `num-traits` as a direct dependency.

# 3.5.4

> Feb 27, 2021

- Support of `cgmath-0.18`.

# 3.5.3

> Jan 16, 2021

- Resynchronize and fix links in the README (fix in `cargo sync-readme`).

# 3.5.2

> Fri Jan 01, 2021

- Support of `nalgebra-0.24`.

# 3.5.1

> Dec 5th, 2020

- Support of `glam-0.11`.

# 3.5

> Nov 23rd, 2020

- Add support for [glam](https://crates.io/crates/glam) via the `"impl-glam"` feature gate.
- Support of `nalgebra-0.23`.

# 3.4.2

> Oct 24th, 2020

- Support of `simba-0.3`.

# 3.4.1

> Sep 5th, 2020

- Support of `simba-0.2`.
- Support of `nalgebra-0.22`.

# 3.4

> Thu May 21st 2020

- Add support for `float-cmp-0.7` and `float-cmp-0.8`. Because this uses a SemVer range, if you
  already have a `Cargo.lock`, don’t forget to update `splines` with `cargo update --aggressive`.

# 3.3

> Thu Apr 10th 2020

- Add support for `nalgebra-0.21`.

# 3.2

> Thu Mar 19th 2020

- Add support for `nalgebra-0.20`.
- Add support for `float-cmp-0.6`.

# 3.1

> Sat Jan 26th 2020

- Add support for `nalgebra-0.19`.

# 3.0

> Tue Oct 22th 2019

## Major changes

- Sampling now requires the value of the key to be `Linear<T>` for `Interpolate<T>`. That is needed
  to ease some interpolation mode (especially Bézier).

## Patch changes

- Fix Bézier interpolation when the next key is Bézier too.

# 2.2

> Mon Oct 17th 2019

- Add `Interpolation::StrokeBezier`.

# 2.1.1

> Mon Oct 17th 2019

- Licensing support in the crate.

# 2.1

> Mon Sep 30th 2019

- Add `Spline::sample_with_key` and `Spline::clamped_sample_with_key`. Those methods allow one to
  perform the regular `Spline::sample` and `Spline::clamped_sample` but also retreive the base
  key that was used to perform the interpolation. The key can be inspected to get the base time,
  interpolation, etc. The next key is also returned, if present.

# 2.0.1

> Tue Sep 24th 2019

- Fix the cubic Bézier curve interpolation. The “output” tangent is now taken by mirroring the
  next key’s tangent around its control point.

# 2.0

> Mon Sep 23rd 2019

## Major changes

- Add support for [Bézier curves](https://en.wikipedia.org/wiki/B%C3%A9zier_curve).
- Because of Bézier curves, the `Interpolation` type now has one more type variable to know how we
  should interpolate with Bézier.

## Minor changes

- Add `Spline::get`, `Spline::get_mut` and `Spline::replace`.

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
