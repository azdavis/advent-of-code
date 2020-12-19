#[derive(Debug, Clone)]
pub struct Mem<T> {
  inner: Vec<T>,
}

impl<T> Mem<T> {
  pub fn new(vec: Vec<T>) -> Self {
    Self { inner: vec }
  }

  pub fn write(&mut self, addr: usize, val: T) {
    self.inner[addr] = val;
  }
}

impl<T> Mem<T>
where
  T: Copy,
{
  pub fn read(&self, addr: usize) -> T {
    self.inner[addr]
  }
}
