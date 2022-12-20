use helpers::{HashMap, HashSet};

type Coord = [i32; 3];
type Scanner = HashSet<Coord>;

fn parse(s: &str) -> HashMap<u8, Scanner> {
  let mut lines = s.lines();
  let mut ret = HashMap::<u8, Scanner>::default();
  let mut idx = 0u8;
  while lines.next().is_some() {
    let mut scanner = Scanner::default();
    for line in lines.by_ref() {
      if line.is_empty() {
        break;
      }
      let mut parts = line.split(',');
      let x: i32 = parts.next().unwrap().parse().unwrap();
      let y: i32 = parts.next().unwrap().parse().unwrap();
      let z: i32 = parts.next().unwrap().parse().unwrap();
      scanner.insert([x, y, z]);
    }
    ret.insert(idx, scanner);
    idx += 1;
  }
  ret
}

fn all_translations(sc: &Scanner) -> impl Iterator<Item = Scanner> + '_ {
  COORD_TRANSLATIONS
    .into_iter()
    .map(|f| sc.iter().map(|&c| f(c)).collect())
}

/// linear algebra? _shrug_
const COORD_TRANSLATIONS: [fn(Coord) -> Coord; 24] = [
  |[x, y, z]| [x, y, z],
  |[x, y, z]| [x, z, -y],
  |[x, y, z]| [x, -y, -z],
  |[x, y, z]| [x, -z, y],
  |[x, y, z]| [-x, y, -z],
  |[x, y, z]| [-x, -z, -y],
  |[x, y, z]| [-x, -y, z],
  |[x, y, z]| [-x, z, y],
  |[x, y, z]| [y, -x, z],
  |[x, y, z]| [y, z, x],
  |[x, y, z]| [y, x, -z],
  |[x, y, z]| [y, -z, -x],
  |[x, y, z]| [-y, x, z],
  |[x, y, z]| [-y, z, -x],
  |[x, y, z]| [-y, -x, -z],
  |[x, y, z]| [-y, -z, x],
  |[x, y, z]| [z, y, -x],
  |[x, y, z]| [z, -x, -y],
  |[x, y, z]| [z, -y, x],
  |[x, y, z]| [z, x, y],
  |[x, y, z]| [-z, y, x],
  |[x, y, z]| [-z, x, -y],
  |[x, y, z]| [-z, -y, -x],
  |[x, y, z]| [-z, -x, y],
];

fn align(sc: &Scanner, a: Coord, b: Coord) -> (Coord, Scanner) {
  assert!(sc.contains(&a));
  let [ax, ay, az] = a;
  let [bx, by, bz] = b;
  let dx = bx - ax;
  let dy = by - ay;
  let dz = bz - az;
  let ret_sc: Scanner = sc
    .iter()
    .map(|&[x, y, z]| [x + dx, y + dy, z + dz])
    .collect();
  ([dx, dy, dz], ret_sc)
}

const THRESHOLD: usize = 12;

/// slow AF.
fn run(s: &str) -> Vec<(Coord, Scanner)> {
  let mut scanners = parse(s);
  let mut ret: Vec<(Coord, Scanner)> = vec![([0, 0, 0], scanners.remove(&0).unwrap())];
  while !scanners.is_empty() {
    let (idx, offset, sc) = scanners
      .iter()
      .find_map(|(&idx, sc)| {
        all_translations(sc).find_map(|tr_sc| {
          ret.iter().find_map(|(_, kn_sc)| {
            kn_sc.iter().find_map(|&kn_sc_pt| {
              tr_sc.iter().find_map(|&tr_sc_pt| {
                let (offset, new_tr_sc) = align(&tr_sc, tr_sc_pt, kn_sc_pt);
                let overlaps = kn_sc.intersection(&new_tr_sc).count();
                (overlaps >= THRESHOLD).then_some((idx, offset, new_tr_sc))
              })
            })
          })
        })
      })
      .unwrap();
    scanners.remove(&idx);
    ret.push((offset, sc));
  }
  ret
}

fn get_p1(res: &[(Coord, Scanner)]) -> usize {
  let set: HashSet<_> = res.iter().flat_map(|(_, sc)| sc).collect();
  set.len()
}

pub fn p1(s: &str) -> usize {
  get_p1(&run(s))
}

fn city_dist(a: Coord, b: Coord) -> i32 {
  let [ax, ay, az] = a;
  let [bx, by, bz] = b;
  (ax - bx).abs() + (ay - by).abs() + (az - bz).abs()
}

fn get_p2(res: &[(Coord, Scanner)]) -> i32 {
  let offsets: Vec<_> = res.iter().map(|&(off, _)| off).collect();
  offsets
    .iter()
    .flat_map(|&a| offsets.iter().map(move |&b| city_dist(a, b)))
    .max()
    .unwrap()
}

pub fn p2(s: &str) -> i32 {
  get_p2(&run(s))
}

#[test]
fn t() {
  let s = include_str!("input/d19.txt");
  // speed up the tests ~2x
  let res = run(s);
  assert_eq!(get_p1(&res), 396);
  assert_eq!(get_p2(&res), 11828);
}

#[test]
fn ex2() {
  let s = include_str!("input/d19_ex2.txt");
  let res = run(s);
  assert_eq!(get_p1(&res), 79);
  assert_eq!(get_p2(&res), 3621);
}
