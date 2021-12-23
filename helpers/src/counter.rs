use crate::HashMap;
use std::hash::Hash;
use std::iter::IntoIterator;

/// A counter of `T`s.
#[derive(Debug, Clone)]
pub struct Counter<T> {
  map: HashMap<T, usize>,
}

impl<T> Counter<T>
where
  T: Hash + Eq,
{
  /// Returns a new counter with the given capacity.
  pub fn with_capacity(cap: usize) -> Counter<T> {
    Counter {
      map: HashMap::with_capacity_and_hasher(cap, Default::default()),
    }
  }

  /// Returns the count for `key`. Defaults to 0.
  pub fn get(&self, key: T) -> usize {
    self.map.get(&key).copied().unwrap_or_default()
  }

  /// Adds `num` to the count for `key`. Returns the new count.
  pub fn add(&mut self, key: T, num: usize) -> usize {
    let val = self.map.entry(key).or_default();
    let ret = *val + num;
    *val = ret;
    ret
  }

  /// Adds 1 to the count for `key`. Returns the new count.
  pub fn inc(&mut self, key: T) -> usize {
    self.add(key, 1)
  }

  /// Returns the length of this counter.
  pub fn len(&self) -> usize {
    self.map.len()
  }

  /// Returns whether this counter is empty.
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Returns an iterator over the keys and their counts.
  pub fn iter(&self) -> impl Iterator<Item = (&T, usize)> {
    self.map.iter().map(|(k, &v)| (k, v))
  }
}

impl<T> Default for Counter<T> {
  fn default() -> Counter<T> {
    Counter {
      map: HashMap::default(),
    }
  }
}

impl<T> PartialEq for Counter<T>
where
  T: Hash + Eq,
{
  fn eq(&self, other: &Self) -> bool {
    self.map == other.map
  }
}

impl<T> Eq for Counter<T> where T: Hash + Eq {}

/// An owning iterator over a [`Counter`].
#[derive(Debug)]
pub struct IntoIter<T> {
  inner: std::collections::hash_map::IntoIter<T, usize>,
}

impl<T> Iterator for IntoIter<T> {
  type Item = (T, usize);
  fn next(&mut self) -> Option<Self::Item> {
    self.inner.next()
  }
}

impl<T> IntoIterator for Counter<T> {
  type Item = (T, usize);

  type IntoIter = IntoIter<T>;

  fn into_iter(self) -> Self::IntoIter {
    IntoIter {
      inner: self.map.into_iter(),
    }
  }
}
