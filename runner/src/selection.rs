use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct SelectionError {
  pub year: u16,
  pub day: u16,
  pub part: u16,
}

impl fmt::Display for SelectionError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "invalid selection: year {}, day {}, part {}",
      self.year, self.day, self.part
    )
  }
}

impl Error for SelectionError {}
