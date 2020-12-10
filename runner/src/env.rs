use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct VarError {
  name: &'static str,
  error: std::env::VarError,
}

impl fmt::Display for VarError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "couldn't get env var {}: {}", self.name, self.error)
  }
}

impl Error for VarError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    Some(&self.error)
  }
}

pub fn var(name: &'static str) -> Result<String, VarError> {
  std::env::var(name).map_err(|error| VarError { name, error })
}
