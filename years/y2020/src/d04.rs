use maplit::hashset;

pub fn p1(s: &str) {
  let mut ok = 0;
  for passport in s.split("\n\n") {
    let mut need = hashset!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for field in passport.split_ascii_whitespace() {
      let field = field.split(':').next().unwrap();
      need.remove(field);
    }
    if need.is_empty() {
      ok += 1;
    }
  }
  println!("{}", ok);
}
