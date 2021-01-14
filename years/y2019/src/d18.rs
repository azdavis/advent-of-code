use std::collections::{HashSet, VecDeque};

pub fn p1(s: &str) -> usize {
  let (maze, loc) = parse(s);
  // depends on no dupe keys
  let num_keys: usize = maze
    .iter()
    .map(|row| {
      row
        .iter()
        .filter(|&&tile| matches!(tile, Tile::Key(_)))
        .count()
    })
    .sum();
  let mut states = vec![State {
    loc,
    keys: HashSet::new(),
    steps: 0,
  }];
  let mut queue = VecDeque::new();
  let mut visited = HashSet::new();
  let mut min_steps: Option<usize> = None;
  while let Some(st) = states.pop() {
    if st.keys.len() == num_keys {
      min_steps = Some(min_steps.map_or(st.steps, |ac| ac.min(st.steps)));
      continue;
    }
    queue.clear();
    visited.clear();
    queue.push_back(st.loc);
    let mut steps = 0;
    while !queue.is_empty() {
      for _ in 0..queue.len() {
        let loc = queue.pop_front().unwrap();
        if !visited.insert(loc) {
          continue;
        }
        match maze[loc.y][loc.x] {
          Tile::Wall => continue,
          Tile::Open => {}
          Tile::Key(k) => {
            if !st.keys.contains(&k) {
              let mut keys = st.keys.clone();
              keys.insert(k);
              states.push(State {
                loc,
                keys,
                steps: st.steps + steps,
              });
              continue;
            }
          }
          Tile::Door(d) => {
            if !st.keys.contains(&d) {
              continue;
            }
          }
        }
        let x = loc.x;
        let y = loc.y;
        let neighbors = [
          x.checked_sub(1).map(|x| Vec2 { x, y }),
          x.checked_add(1).map(|x| Vec2 { x, y }),
          y.checked_sub(1).map(|y| Vec2 { x, y }),
          y.checked_add(1).map(|y| Vec2 { x, y }),
        ];
        queue.extend(neighbors.iter().flatten().copied());
      }
      steps += 1;
    }
  }
  min_steps.unwrap()
}

pub fn p2(_: &str) -> u32 {
  todo!()
}

#[derive(Debug, Clone, Copy)]
enum Tile {
  Wall,
  Open,
  Key(u8),
  Door(u8),
}

/// use usize
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
  x: usize,
  y: usize,
}

#[derive(Debug)]
struct State {
  loc: Vec2,
  keys: HashSet<u8>,
  steps: usize,
}

fn parse(s: &str) -> (Vec<Vec<Tile>>, Vec2) {
  let mut cur: Option<Vec2> = None;
  let mut maze = Vec::<Vec<Tile>>::new();
  for (y, line) in s.lines().enumerate() {
    let mut row = Vec::<Tile>::new();
    for (x, b) in line.bytes().enumerate() {
      let tile = match b {
        b'@' => {
          cur = Some(Vec2 { x, y });
          Tile::Open
        }
        b'.' => Tile::Open,
        b'#' => Tile::Wall,
        _ => {
          if b.is_ascii_lowercase() {
            Tile::Key(b)
          } else if b.is_ascii_uppercase() {
            Tile::Door(b.to_ascii_lowercase())
          } else {
            panic!("bad byte: {}", b)
          }
        }
      };
      row.push(tile);
    }
    maze.push(row);
  }
  (maze, cur.unwrap())
}

#[test]
fn t() {
  // let s = include_str!("input/d18.txt");
  // assert_eq!(p1(s), ___);
  // assert_eq!(p2(s), ___);
}

#[test]
fn t_p1() {
  let s = include_str!("input/d18_ex1.txt");
  assert_eq!(p1(s), 8);
  let s = include_str!("input/d18_ex2.txt");
  assert_eq!(p1(s), 86);
}
