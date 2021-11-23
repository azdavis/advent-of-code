//! Helpers that are generic enough that they may be used across various
//! problems, as well as re-exports of dependencies.

#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(rust_2018_idioms)]

mod bit_set;
pub mod block_char;
mod ceil_div;
mod compass;
pub mod digits;
pub mod dijkstra;
mod gcd;
mod infinitable;
pub mod matrix;
mod permute;
mod vec2;

pub use bit_set::BitSet;
pub use ceil_div::ceil_div;
pub use compass::Compass;
pub use float_ord::{sort, FloatOrd};
pub use gcd::{gcd, lcm};
pub use infinitable::Infinitable;
pub use once_cell::sync::Lazy;
pub use permute::permute;
pub use regex::Regex;
pub use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
pub use vec2::Vec2;

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
