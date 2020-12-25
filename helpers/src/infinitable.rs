//! Possibly-infinite values.

use std::ops::Add;

/// A possibly (negatively) infinite value of type `T`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Infinitable<T> {
  /// Negative infinity.
  NegInf,
  /// A finite value `T`.
  Finite(T),
  /// Positive infinity.
  PosInf,
}

impl<T, U> Add<U> for Infinitable<T>
where
  T: Add<U>,
{
  type Output = Infinitable<T::Output>;

  fn add(self, rhs: U) -> Self::Output {
    match self {
      Infinitable::NegInf => Infinitable::NegInf,
      Infinitable::Finite(x) => Infinitable::Finite(x + rhs),
      Infinitable::PosInf => Infinitable::PosInf,
    }
  }
}

#[test]
fn partial_ord() {
  assert_eq!(Infinitable::Finite(3), Infinitable::Finite(3));
  assert!(Infinitable::Finite(3) < Infinitable::Finite(4));
  assert!(Infinitable::Finite(5) > Infinitable::Finite(4));
  assert!(Infinitable::PosInf > Infinitable::Finite(i32::MAX));
  assert!(Infinitable::NegInf < Infinitable::Finite(i32::MIN));
  assert!(Infinitable::<i32>::NegInf < Infinitable::PosInf);
}

#[test]
fn add() {
  assert_eq!(Infinitable::Finite(3) + 4, Infinitable::Finite(7));
  assert_eq!(Infinitable::<i32>::PosInf + 4, Infinitable::PosInf);
  assert_eq!(Infinitable::<i32>::NegInf + 4, Infinitable::NegInf);
}
