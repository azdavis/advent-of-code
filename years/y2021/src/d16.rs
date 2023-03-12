struct Packet {
  version: usize,
  kind: Kind,
}

enum Kind {
  Lit(usize),
  Add(Vec<Packet>),
  Mul(Vec<Packet>),
  Min(Vec<Packet>),
  Max(Vec<Packet>),
  Gt(Box<Packet>, Box<Packet>),
  Lt(Box<Packet>, Box<Packet>),
  Eq(Box<Packet>, Box<Packet>),
}

fn parse_num(iter: &mut dyn Iterator<Item = bool>, digits: usize) -> usize {
  iter
    .take(digits)
    .fold(0, |ac, x| (ac << 1) | usize::from(x))
}

fn get_sub_packets(iter: &mut dyn Iterator<Item = bool>) -> Vec<Packet> {
  let mut ret = Vec::<Packet>::new();
  let length_type_id = iter.next().unwrap();
  if length_type_id {
    let num_sub_packets = parse_num(iter, 11);
    for _ in 0..num_sub_packets {
      ret.push(parse_packet(iter));
    }
  } else {
    let bit_length = parse_num(iter, 15);
    let mut sub_bits = iter.take(bit_length).peekable();
    while sub_bits.peek().is_some() {
      ret.push(parse_packet(&mut sub_bits));
    }
  }
  ret
}

fn get_two<T>(xs: Vec<T>) -> [Box<T>; 2] {
  let mut iter = xs.into_iter();
  let fst = iter.next().unwrap();
  let snd = iter.next().unwrap();
  assert!(iter.next().is_none());
  [Box::new(fst), Box::new(snd)]
}

fn parse_packet(iter: &mut dyn Iterator<Item = bool>) -> Packet {
  let version = parse_num(iter, 3);
  let type_id = parse_num(iter, 3);
  let kind = match type_id {
    0 => Kind::Add(get_sub_packets(iter)),
    1 => Kind::Mul(get_sub_packets(iter)),
    2 => Kind::Min(get_sub_packets(iter)),
    3 => Kind::Max(get_sub_packets(iter)),
    4 => {
      let mut lit = 0usize;
      loop {
        let cont = iter.next().unwrap();
        lit = (lit << 4) | parse_num(iter, 4);
        if !cont {
          break Kind::Lit(lit);
        }
      }
    }
    5 => {
      let [lhs, rhs] = get_two(get_sub_packets(iter));
      Kind::Gt(lhs, rhs)
    }
    6 => {
      let [lhs, rhs] = get_two(get_sub_packets(iter));
      Kind::Lt(lhs, rhs)
    }
    7 => {
      let [lhs, rhs] = get_two(get_sub_packets(iter));
      Kind::Eq(lhs, rhs)
    }
    _ => panic!("unknown type id: {type_id}"),
  };
  Packet { version, kind }
}

fn sum_versions(p: &Packet) -> usize {
  let sub_packet_sum = match &p.kind {
    Kind::Lit(_) => 0,
    Kind::Add(ps) | Kind::Mul(ps) | Kind::Min(ps) | Kind::Max(ps) => {
      ps.iter().map(sum_versions).sum()
    }
    Kind::Gt(lhs, rhs) | Kind::Lt(lhs, rhs) | Kind::Eq(lhs, rhs) => {
      sum_versions(lhs) + sum_versions(rhs)
    }
  };
  p.version + sub_packet_sum
}

fn eval_packet(p: &Packet) -> usize {
  match &p.kind {
    Kind::Lit(n) => *n,
    Kind::Add(ps) => ps.iter().map(eval_packet).sum(),
    Kind::Mul(ps) => ps.iter().map(eval_packet).product(),
    Kind::Min(ps) => ps.iter().map(eval_packet).min().unwrap(),
    Kind::Max(ps) => ps.iter().map(eval_packet).max().unwrap(),
    Kind::Gt(lhs, rhs) => usize::from(eval_packet(lhs) > eval_packet(rhs)),
    Kind::Lt(lhs, rhs) => usize::from(eval_packet(lhs) < eval_packet(rhs)),
    Kind::Eq(lhs, rhs) => usize::from(eval_packet(lhs) == eval_packet(rhs)),
  }
}

fn parse_bits(s: &str) -> impl Iterator<Item = bool> + '_ {
  s.trim().chars().flat_map(|c| {
    let digit = c.to_digit(16).unwrap();
    (0..4).rev().map(move |s| {
      let mask = 1 << s;
      (digit & mask) == mask
    })
  })
}

pub fn p1(s: &str) -> usize {
  sum_versions(&parse_packet(&mut parse_bits(s)))
}

pub fn p2(s: &str) -> usize {
  eval_packet(&parse_packet(&mut parse_bits(s)))
}

#[test]
fn t() {
  let s = include_str!("input/d16.txt");
  assert_eq!(p1(s), 943);
  assert_eq!(p2(s), 167_737_115_857);
}

#[test]
fn ex() {
  assert_eq!(p1("8A004A801A8002F478"), 16);
  assert_eq!(p1("620080001611562C8802118E34"), 12);
  assert_eq!(p1("C0015000016115A2E0802F182340"), 23);
  assert_eq!(p1("A0016C880162017C3686B18A3D4780"), 31);
}
