//! A set of bits.

/// A set of bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BitSet {
  inner: u32,
}

impl BitSet {
  /// The maximum value that can be stored in this set.
  pub const MAX: u8 = 31;

  /// Returns an empty `BitSet`.
  pub fn new() -> Self {
    Default::default()
  }

  /// Adds `val` to the set. Returns whether `val` was _not_ in the set. Panics
  /// if `val > Self::MAX`.
  pub fn insert(&mut self, val: u8) -> bool {
    let (contained, s) = self.contains_impl(val);
    self.inner |= s;
    !contained
  }

  /// Removes `val` from the set. Returns whether `val` was in the set. Panics
  /// if `val > Self::MAX`.
  pub fn remove(&mut self, val: u8) -> bool {
    let (contained, s) = self.contains_impl(val);
    self.inner &= !s;
    contained
  }

  /// Returns whether `val` is in the set. Panics if `val > Self::MAX`.
  pub fn contains(&self, val: u8) -> bool {
    self.contains_impl(val).0
  }

  #[inline]
  fn contains_impl(&self, val: u8) -> (bool, u32) {
    assert!(val <= Self::MAX);
    let s = 1 << val;
    ((self.inner & s) == s, s)
  }

  /// Returns the number of values in the set.
  pub fn len(&self) -> u32 {
    self.inner.count_ones()
  }

  /// Returns whether the set is empty.
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Returns whether `self` is a subset of `other`.
  pub fn is_subset(&self, other: &Self) -> bool {
    (self.inner & !other.inner) == 0
  }

  /// Returns an iterator over the elements in the set in ascending order.
  pub fn iter(&self) -> Iter {
    Iter {
      set: *self,
      counter: 0,
    }
  }
}

/// An iterator over the elements in the set in ascending order.
#[derive(Debug, Clone, Copy)]
pub struct Iter {
  set: BitSet,
  counter: u8,
}

impl Iterator for Iter {
  type Item = u8;
  fn next(&mut self) -> Option<Self::Item> {
    while self.counter <= BitSet::MAX {
      let next = self.counter;
      self.counter += 1;
      if self.set.contains(next) {
        return Some(next);
      }
    }
    None
  }
}

#[test]
fn t() {
  let mut a = BitSet::new();
  let mut b = BitSet::new();
  assert_eq!(a.len(), 0);
  assert_eq!(b.len(), 0);
  assert!(a.insert(3));
  assert!(b.insert(3));
  assert_eq!(a.len(), 1);
  assert_eq!(b.len(), 1);
  assert!(!a.insert(3));
  assert!(!b.insert(3));
  assert_eq!(a.len(), 1);
  assert_eq!(b.len(), 1);
  assert!(b.is_subset(&a));
  assert!(a.is_subset(&b));
  assert!(a.contains(3));
  assert!(!a.contains(4));
  assert!(!b.contains(5));
  assert!(b.insert(5));
  assert!(a.is_subset(&b));
  assert!(!b.is_subset(&a));
  assert_eq!(b.len(), 2);
  assert!(b.contains(3));
  assert!(b.contains(5));
  assert_eq!(a.iter().collect::<Vec<_>>(), [3]);
  assert_eq!(b.iter().collect::<Vec<_>>(), [3, 5]);
  assert!(!a.remove(4));
  assert!(!a.remove(5));
  assert!(a.remove(3));
  assert!(!a.remove(3));
  assert!(a.is_empty());
}
