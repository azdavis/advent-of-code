//! See [`CycleZipper`].

/// A cyclic data structure, into which we have a 1-element wide window we can
/// access and move around in O(1) amortized time (I think).
pub(super) struct CycleZipper<T> {
  /// in 'regular' order
  front: Vec<T>,
  /// in reverse order
  rear: Vec<T>,
}

impl<T> CycleZipper<T> {
  /// Returns a new [`CycleZipper`] containing the single element, which becomes
  /// the current element.
  pub(super) fn new(init: T) -> CycleZipper<T> {
    CycleZipper {
      front: vec![init],
      rear: vec![],
    }
  }

  /// Sets the current element to the next element.
  pub(super) fn move_next(&mut self) {
    if self.front.is_empty() {
      self.rearrange(0);
    }
    let val = self.front.pop().unwrap();
    self.rear.push(val);
  }

  /// Sets the current element to the previous element.
  pub(super) fn move_prev(&mut self) {
    if self.rear.is_empty() {
      self.rearrange(1);
    }
    let val = self.rear.pop().unwrap();
    self.front.push(val);
  }

  /// Inserts an element directly after the current element, and makes that the
  /// current element.
  pub(super) fn push(&mut self, val: T) {
    self.front.push(val);
  }

  /// Removes and returns the current element.
  ///
  /// Bad things happen if the element was the last element in this
  /// [`CycleZipper`].
  pub(super) fn pop(&mut self) -> T {
    if self.front.is_empty() {
      self.rearrange(0);
    }
    self.front.pop().unwrap()
  }

  /// arrange for `self.front` and `self.rear` to contain approximately the same
  /// number of elements, while preserving the logical order of elements. this
  /// should make the move/push/pop operations O(1) amortized, I believe.
  fn rearrange(&mut self, bias: usize) {
    self.front.extend(self.rear.drain(..).rev());
    let half = self.front.len() / 2;
    let new_front = self.front.split_off(half + bias);
    std::mem::swap(&mut self.rear, &mut self.front);
    self.rear.reverse();
    self.front = new_front;
  }
}
