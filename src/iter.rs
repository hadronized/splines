use crate::{Key, Spline};

/// Iterator over spline keys.
///
/// This iterator type assures you to iterate over sorted keys.
pub struct Iter<'a, T, V> where T: 'a, V: 'a {
  anim_param: &'a Spline<T, V>,
  i: usize
}

impl<'a, T, V> Iterator for Iter<'a, T, V> {
  type Item = &'a Key<T, V>;

  fn next(&mut self) -> Option<Self::Item> {
    let r = self.anim_param.0.get(self.i);

    if let Some(_) = r {
      self.i += 1;
    }

    r
  }
}

impl<'a, T, V> IntoIterator for &'a Spline<T, V> {
  type Item = &'a Key<T, V>;
  type IntoIter = Iter<'a, T, V>;

  fn into_iter(self) -> Self::IntoIter {
    Iter {
      anim_param: self,
      i: 0
    }
  }
}

