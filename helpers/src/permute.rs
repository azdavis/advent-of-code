//! Permutations.

/// Runs [Heap's algorithm][1], which generates the permutations of `xs`.
///
/// [1]: https://en.wikipedia.org/wiki/Heap%27s_algorithm
pub fn permute<I, T>(iter: I) -> Vec<Vec<T>>
where
  I: IntoIterator<Item = T>,
  T: Clone,
{
  let mut xs: Vec<_> = iter.into_iter().collect();
  let mut ret = vec![xs.clone()];
  let mut c = vec![0; xs.len()];
  let mut i = 0;
  while i < xs.len() {
    if c[i] < i {
      if i % 2 == 0 {
        xs.swap(0, i);
      } else {
        xs.swap(c[i], i);
      }
      ret.push(xs.clone());
      c[i] += 1;
      i = 0;
    } else {
      c[i] = 0;
      i += 1;
    }
  }
  ret
}

#[test]
fn t() {
  assert_eq!(permute(Vec::<u32>::new()), [[]]);
  assert_eq!(permute(vec![3]), [[3]]);
  assert_eq!(permute(vec![1, 2]), [[1, 2], [2, 1]]);
  assert_eq!(
    permute(vec![1, 2, 3]),
    [
      [1, 2, 3],
      [2, 1, 3],
      [3, 1, 2],
      [1, 3, 2],
      [2, 3, 1],
      [3, 2, 1]
    ]
  );
}
