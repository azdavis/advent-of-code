use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Mem<T> {
  inner: HashMap<usize, T>,
}

impl<T> Mem<T> {
  pub fn new<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = T>,
  {
    Self {
      inner: iter.into_iter().enumerate().collect(),
    }
  }

  pub fn write(&mut self, addr: usize, val: T) {
    self.inner.insert(addr, val);
  }
}

impl<T> Mem<T>
where
  T: Copy + Default,
{
  pub fn read(&self, addr: usize) -> T {
    self.inner.get(&addr).copied().unwrap_or_default()
  }
}
