use helpers::maplit::hashset;
use helpers::once_cell::sync::Lazy;
use helpers::regex::Regex;

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

static HEIGHT: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+)(\w+)$").unwrap());
static HAIR_COLOR: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"^#[0-9a-f]{6}$").unwrap());
static PASSPORT_ID: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{9}$").unwrap());

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
        "cm" => 150 <= n && n <= 193,
        "in" => 59 <= n && n <= 76,
        _ => false,
      }
    }
    Key::HairColor => HAIR_COLOR.is_match(val),
    Key::EyeColor => {
      matches!(val, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
    }
    Key::PassportID => PASSPORT_ID.is_match(val),
    Key::CountryID => true,
  }
}

fn go<F>(s: &str, f: F) -> usize
where
  F: Fn(Key, &str) -> bool,
{
  let mut ret = 0;
  'outer: for passport in s.split("\n\n") {
    let mut need = hashset![
      Key::BirthYear,
      Key::IssueYear,
      Key::ExpirationYear,
      Key::Height,
      Key::HairColor,
      Key::EyeColor,
      Key::PassportID,
    ];
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
  PassportID,
  CountryID,
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
      "pid" => Self::PassportID,
      "cid" => Self::CountryID,
      _ => panic!("bad key: {}", s),
    }
  }
}

#[test]
fn t() {
  let inp = include_str!("input/d04.txt");
  assert_eq!(p1(inp), 233);
  assert_eq!(p2(inp), 111);
}
