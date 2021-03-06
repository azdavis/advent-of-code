//! Helpers that are generic enough that they may be used across various
//! problems, as well as re-exports of dependencies.

#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(rust_2018_idioms)]

pub mod bit_set;
pub mod block_char;
pub mod ceil_div;
pub mod compass;
pub mod digits;
pub mod gcd;
pub mod infinitable;
pub mod matrix;
pub mod permute;
pub mod vec2;

pub use float_ord;
pub use maplit;
pub use once_cell;
pub use regex;
