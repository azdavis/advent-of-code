//! Possibly-infinite values.

/// A possibly (negatively) infinite value of type `T`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Infinite<T> {
  NegInf,
  Finite(T),
  PosInf,
}

#[test]
fn partial_ord() {
  assert_eq!(Infinite::Finite(3), Infinite::Finite(3));
  assert!(Infinite::Finite(3) < Infinite::Finite(4));
  assert!(Infinite::Finite(5) > Infinite::Finite(4));
  assert!(Infinite::PosInf > Infinite::Finite(i32::MAX));
  assert!(Infinite::NegInf < Infinite::Finite(i32::MIN));
  assert!(Infinite::<i32>::NegInf < Infinite::PosInf);
}
