//! Available interpolation modes.

#[cfg(feature = "serialization")] use serde_derive::{Deserialize, Serialize};

/// Available kind of interpolations.
///
/// Feel free to visit each variant for more documentation.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serialization", serde(rename_all = "snake_case"))]
pub enum Interpolation<T> {
  /// Hold a [`Key<T, _>`] until the sampling value passes the normalized step threshold, in which
  /// case the next key is used.
  ///
  /// > Note: if you set the threshold to `0.5`, the first key will be used until half the time
  /// > between the two keys; the second key will be in used afterwards. If you set it to `1.0`, the
  /// > first key will be kept until the next key. Set it to `0.` and the first key will never be
  /// > used.
  ///
  /// [`Key<T, _>`]: crate::key::Key
  Step(T),
  /// Linear interpolation between a key and the next one.
  Linear,
  /// Cosine interpolation between a key and the next one.
  Cosine,
  /// Catmull-Rom interpolation, performing a cubic Hermite interpolation using four keys.
  CatmullRom
}

impl<T> Default for Interpolation<T> {
  /// [`Interpolation::Linear`] is the default.
  fn default() -> Self {
    Interpolation::Linear
  }
}

