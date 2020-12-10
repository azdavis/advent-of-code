use maplit::hashset;

pub fn p1(s: &str) {
  help(s, |_, _| true)
}

fn help<F>(s: &str, f: F)
where
  F: Fn(&str, &str) -> bool,
{
  let mut ok = 0;
  'outer: for passport in s.split("\n\n") {
    let mut need = hashset!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for field in passport.split_ascii_whitespace() {
      let mut iter = field.split(':');
      let key = iter.next().unwrap();
      let val = iter.next().unwrap();
      assert!(iter.next().is_none());
      if !f(key, val) {
        continue 'outer;
      }
      need.remove(key);
    }
    if need.is_empty() {
      ok += 1;
    }
  }
  println!("{}", ok);
}
