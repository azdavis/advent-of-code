//! Possibly-infinite values.

use std::ops::Add;

/// A possibly (negatively) infinite value of type `T`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Infinite<T> {
  NegInf,
  Finite(T),
  PosInf,
}

impl<T, U> Add<U> for Infinite<T>
where
  T: Add<U>,
{
  type Output = Infinite<T::Output>;

  fn add(self, rhs: U) -> Self::Output {
    match self {
      Infinite::NegInf => Infinite::NegInf,
      Infinite::Finite(x) => Infinite::Finite(x + rhs),
      Infinite::PosInf => Infinite::PosInf,
    }
  }
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

#[test]
fn add() {
  assert_eq!(Infinite::Finite(3) + 4, Infinite::Finite(7));
  assert_eq!(Infinite::<i32>::PosInf + 4, Infinite::PosInf);
  assert_eq!(Infinite::<i32>::NegInf + 4, Infinite::NegInf);
}
