//! A set of bits.

/// A set of bits.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct BitSet {
  inner: u32,
}

impl BitSet {
  /// The maximum value that can be stored in this set.
  pub(super) const MAX: u8 = 31;

  /// Adds `val` to the set. Returns whether `val` was _not_ in the set. Panics
  /// if `val > Self::MAX`.
  pub(super) fn insert(&mut self, val: u8) -> bool {
    let (contained, s) = self.contains_impl(val);
    self.inner |= s;
    !contained
  }

  /// Returns whether `val` is in the set. Panics if `val > Self::MAX`.
  pub(super) fn contains(self, val: u8) -> bool {
    self.contains_impl(val).0
  }

  #[inline]
  fn contains_impl(self, val: u8) -> (bool, u32) {
    assert!(val <= Self::MAX);
    let s = 1 << val;
    ((self.inner & s) == s, s)
  }

  /// Returns the number of values in the set.
  pub(super) fn len(self) -> u32 {
    self.inner.count_ones()
  }
}

#[test]
fn t() {
  let mut a = BitSet::default();
  let mut b = BitSet::default();
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
  assert!(a.contains(3));
  assert!(!a.contains(4));
  assert!(!b.contains(5));
  assert!(b.insert(5));
  assert_eq!(b.len(), 2);
  assert!(b.contains(3));
  assert!(b.contains(5));
}
