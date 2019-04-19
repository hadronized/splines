#[cfg(feature = "serialization")] use serde_derive::{Deserialize, Serialize};

use crate::interpolation::Interpolation;

/// A spline control point.
///
/// This type associates a value at a given interpolation parameter value. It also contains an
/// interpolation hint used to determine how to interpolate values on the segment defined by this
/// key and the next one – if existing.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serialization", serde(rename_all = "snake_case"))]
pub struct Key<T, V> {
  /// Interpolation parameter at which the [`Key`] should be reached.
  pub t: T,
  /// Held value.
  pub value: V,
  /// Interpolation mode.
  pub interpolation: Interpolation<T>
}

impl<T, V> Key<T, V> {
  /// Create a new key.
  pub fn new(t: T, value: V, interpolation: Interpolation<T>) -> Self {
    Key { t, value, interpolation }
  }
}

