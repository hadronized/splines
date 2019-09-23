//! Available interpolation modes.

#[cfg(feature = "serialization")] use serde_derive::{Deserialize, Serialize};

/// Available kind of interpolations.
///
/// Feel free to visit each variant for more documentation.
#[cfg(feature = "bezier")]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serialization", serde(rename_all = "snake_case"))]
pub enum Interpolation<T, V> {
  /// Hold a [`Key`] until the sampling value passes the normalized step threshold, in which
  /// case the next key is used.
  ///
  /// > Note: if you set the threshold to `0.5`, the first key will be used until half the time
  /// > between the two keys; the second key will be in used afterwards. If you set it to `1.0`, the
  /// > first key will be kept until the next key. Set it to `0.` and the first key will never be
  /// > used.
  ///
  /// [`Key`]: crate::key::Key
  Step(T),
  /// Linear interpolation between a key and the next one.
  Linear,
  /// Cosine interpolation between a key and the next one.
  Cosine,
  /// Catmull-Rom interpolation, performing a cubic Hermite interpolation using four keys.
  CatmullRom,
  /// Bézier interpolation.
  ///
  /// A control point that uses such an interpolation is associated with an extra point. The segmant
  /// connecting both is called the _tangent_ of this point. The part of the spline defined between
  /// this control point and the next one will be interpolated across with Bézier interpolation. Two
  /// cases are possible:
  ///
  /// - The next control point also has a Bézier interpolation mode. In this case, its tangent is
  ///   used for the interpolation process. This is called _cubic Bézier interpolation_ and it
  ///   kicks ass.
  /// - The next control point doesn’t have a Bézier interpolation mode set. In this case, the
  ///   tangent used for the next control point is defined as the segment connecting that control
  ///   point and the current control point’s associated point. This is called _quadratic Bézer
  ///   interpolation_ and it kicks ass too, but a bit less than cubic.
  #[cfg(feature = "bezier")]
  Bezier(V),
}

/// Available kind of interpolations.
///
/// Feel free to visit each variant for more documentation.
#[cfg(not(feature = "bezier"))]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serialization", serde(rename_all = "snake_case"))]
pub enum Interpolation<T> {
  /// Hold a [`Key`] until the sampling value passes the normalized step threshold, in which
  /// case the next key is used.
  ///
  /// > Note: if you set the threshold to `0.5`, the first key will be used until half the time
  /// > between the two keys; the second key will be in used afterwards. If you set it to `1.0`, the
  /// > first key will be kept until the next key. Set it to `0.` and the first key will never be
  /// > used.
  ///
  /// [`Key`]: crate::key::Key
  Step(T),
  /// Linear interpolation between a key and the next one.
  Linear,
  /// Cosine interpolation between a key and the next one.
  Cosine,
  /// Catmull-Rom interpolation, performing a cubic Hermite interpolation using four keys.
  CatmullRom,
}

#[cfg(feature = "bezier")]
impl<T, V> Default for Interpolation<T, V> {
  /// [`Interpolation::Linear`] is the default.
  fn default() -> Self {
    Interpolation::Linear
  }
}

#[cfg(not(feature = "bezier"))]
impl<T> Default for Interpolation<T> {
  /// [`Interpolation::Linear`] is the default.
  fn default() -> Self {
    Interpolation::Linear
  }
}
