use maplit::hashset;

pub fn p1(s: &str) {
  help(s, |_, _| true)
}

fn help<F>(s: &str, f: F)
where
  F: Fn(Key, &str) -> bool,
{
  let mut ok = 0;
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
      let key = Key::parse(iter.next().unwrap()).unwrap();
      let val = iter.next().unwrap();
      assert!(iter.next().is_none());
      if !f(key, val) {
        continue 'outer;
      }
      need.remove(&key);
    }
    if need.is_empty() {
      ok += 1;
    }
  }
  println!("{}", ok);
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
  fn parse(s: &str) -> Option<Key> {
    match s {
      "byr" => Some(Self::BirthYear),
      "iyr" => Some(Self::IssueYear),
      "eyr" => Some(Self::ExpirationYear),
      "hgt" => Some(Self::Height),
      "hcl" => Some(Self::HairColor),
      "ecl" => Some(Self::EyeColor),
      "pid" => Some(Self::PassportID),
      "cid" => Some(Self::CountryID),
      _ => None,
    }
  }
}
