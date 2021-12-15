//! Specialty characters, useful for drawing things in a terminal.

/// A filled box.
pub const FILLED: char = '█';

/// An empty box.
pub const EMPTY: char = '░';

/// - If given `true`, returns [`FILLED`].
/// - If given `false`, returns [`EMPTY`].
pub fn get(b: bool) -> char {
  if b {
    FILLED
  } else {
    EMPTY
  }
}
