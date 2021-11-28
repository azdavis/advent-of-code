struct Tree {
  children: Vec<Tree>,
  metadata: Vec<u32>,
}

fn parse_tree<I>(iter: &mut I) -> Tree
where
  I: Iterator<Item = u32>,
{
  let c = iter.next().unwrap();
  let m = iter.next().unwrap();
  let children: Vec<_> = (0..c).map(|_| parse_tree(iter)).collect();
  let metadata: Vec<_> = (0..m).map(|_| iter.next().unwrap()).collect();
  Tree { children, metadata }
}

fn parse(s: &str) -> Tree {
  let mut iter = s
    .split_ascii_whitespace()
    .map(|s| s.parse::<u32>().unwrap());
  let root = parse_tree(&mut iter);
  assert!(iter.next().is_none());
  root
}

fn sum_metadata(tree: &Tree) -> u32 {
  tree
    .children
    .iter()
    .map(sum_metadata)
    .chain(tree.metadata.iter().copied())
    .sum()
}

pub fn p1(s: &str) -> u32 {
  sum_metadata(&parse(s))
}

fn value(tree: &Tree) -> u32 {
  if tree.children.is_empty() {
    sum_metadata(tree)
  } else {
    tree
      .metadata
      .iter()
      .filter_map(|&n| {
        let n: usize = n.try_into().unwrap();
        Some(value(tree.children.get(n.checked_sub(1)?)?))
      })
      .sum()
  }
}

pub fn p2(s: &str) -> u32 {
  value(&parse(s))
}

#[test]
fn t() {
  let s = include_str!("input/d08.txt");
  assert_eq!(p1(s), 41760);
  assert_eq!(p2(s), 25737);
}
