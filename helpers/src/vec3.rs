//! A length-3 vector (in the mathematics sense).

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec3<T>(pub T, pub T, pub T);
