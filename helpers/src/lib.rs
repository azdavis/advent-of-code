//! Helpers that are generic enough that they may be used across various
//! problems, as well as re-exports of dependencies.

#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

pub mod compass;
pub mod digits;
pub mod gcd;
pub mod infinite;
pub mod matrix;
pub mod permute;
pub mod point;

pub use float_ord;
pub use maplit;
pub use once_cell;
pub use regex;
