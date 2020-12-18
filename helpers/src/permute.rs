//! Permutations.

/// Returns the permutations of `xs` in an arbitrary order.
pub fn permute<T>(xs: Vec<T>) -> Vec<Vec<T>>
where
  T: Clone,
{
  let mut ret = vec![vec![]];
  for x in xs {
    ret = ret
      .into_iter()
      .flat_map(|ys| {
        // eh, not great
        let x = x.clone();
        (0..=ys.len()).map(move |i| {
          let mut ys = ys.clone();
          ys.insert(i, x.clone());
          ys
        })
      })
      .collect();
  }
  ret
}

#[test]
fn t() {
  assert_eq!(permute(Vec::<u32>::new()), vec![vec![]]);
  assert_eq!(permute(vec![3]), vec![vec![3]]);
  assert_eq!(permute(vec![1, 2]), vec![vec![2, 1], vec![1, 2]]);
  assert_eq!(
    permute(vec![1, 2, 3]),
    vec![
      vec![3, 2, 1],
      vec![2, 3, 1],
      vec![2, 1, 3],
      vec![3, 1, 2],
      vec![1, 3, 2],
      vec![1, 2, 3],
    ]
  );
}
