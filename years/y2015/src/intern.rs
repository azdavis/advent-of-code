use helpers::HashMap;

#[derive(Debug, Default)]
pub(crate) struct Intern<'a> {
  map: HashMap<&'a str, usize>,
}

impl<'a> Intern<'a> {
  pub(crate) fn get(&mut self, s: &'a str) -> usize {
    if let Some(&n) = self.map.get(s) {
      return n;
    }
    let ret = self.map.len();
    self.map.insert(s, ret);
    ret
  }

  pub(crate) fn len(&self) -> usize {
    self.map.len()
  }
}
