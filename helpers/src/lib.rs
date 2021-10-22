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
pub use once_cell;
pub use regex;
pub use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

/// Returns a hash set of the given items.
pub fn hash_set<T, const N: usize>(xs: [T; N]) -> HashSet<T>
where
  T: std::hash::Hash + Eq,
{
  xs.into_iter().collect()
}

/// Returns a hash map of the given items.
pub fn hash_map<K, V, const N: usize>(xs: [(K, V); N]) -> HashMap<K, V>
where
  K: std::hash::Hash + Eq,
{
  xs.into_iter().collect()
}
