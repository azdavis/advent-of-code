use helpers::{hash_set, static_regex};

pub fn p1(s: &str) -> usize {
  go(s, |_, _| true)
}

pub fn p2(s: &str) -> usize {
  go(s, is_p2_valid_key)
}

fn is_num_in_range(val: &str, lo: u32, hi: u32) -> bool {
  match val.parse::<u32>() {
    Ok(n) => lo <= n && n <= hi,
    Err(_) => false,
  }
}

static_regex!(HEIGHT = r"^(\d+)(\w+)$");
static_regex!(HAIR_COLOR = r"^#[0-9a-f]{6}$");
static_regex!(PASSPORT_ID = r"^\d{9}$");

fn is_p2_valid_key(key: Key, val: &str) -> bool {
  match key {
    Key::BirthYear => is_num_in_range(val, 1920, 2002),
    Key::IssueYear => is_num_in_range(val, 2010, 2020),
    Key::ExpirationYear => is_num_in_range(val, 2020, 2030),
    Key::Height => {
      let caps = match HEIGHT.captures(val) {
        Some(x) => x,
        None => return false,
      };
      let n: u32 = match caps[1].parse() {
        Ok(x) => x,
        Err(_) => return false,
      };
      match &caps[2] {
        "cm" => (150..=193).contains(&n),
        "in" => (59..=76).contains(&n),
        _ => false,
      }
    }
    Key::HairColor => HAIR_COLOR.is_match(val),
    Key::EyeColor => {
      matches!(val, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
    }
    Key::PassportId => PASSPORT_ID.is_match(val),
    Key::CountryId => true,
  }
}

fn go(s: &str, f: fn(Key, &str) -> bool) -> usize {
  let mut ret = 0;
  'outer: for passport in s.split("\n\n") {
    let mut need = hash_set([
      Key::BirthYear,
      Key::IssueYear,
      Key::ExpirationYear,
      Key::Height,
      Key::HairColor,
      Key::EyeColor,
      Key::PassportId,
    ]);
    for field in passport.split_ascii_whitespace() {
      let mut iter = field.split(':');
      let key = Key::parse(iter.next().unwrap());
      let val = iter.next().unwrap();
      assert!(iter.next().is_none());
      if !f(key, val) {
        continue 'outer;
      }
      need.remove(&key);
    }
    if need.is_empty() {
      ret += 1;
    }
  }
  ret
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Key {
  BirthYear,
  IssueYear,
  ExpirationYear,
  Height,
  HairColor,
  EyeColor,
  PassportId,
  CountryId,
}

impl Key {
  fn parse(s: &str) -> Self {
    match s {
      "byr" => Self::BirthYear,
      "iyr" => Self::IssueYear,
      "eyr" => Self::ExpirationYear,
      "hgt" => Self::Height,
      "hcl" => Self::HairColor,
      "ecl" => Self::EyeColor,
      "pid" => Self::PassportId,
      "cid" => Self::CountryId,
      _ => panic!("bad key: {}", s),
    }
  }
}

#[test]
fn t() {
  let s = include_str!("input/d04.txt");
  assert_eq!(p1(s), 233);
  assert_eq!(p2(s), 111);
}
