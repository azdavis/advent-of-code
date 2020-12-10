mod env;
mod selection;

use selection::SelectionError;
use std::io::Read as _;

fn run() -> Result<(), Box<dyn std::error::Error>> {
  let year: u16 = env::var("AOC_YEAR")?.parse()?;
  let day: u16 = env::var("AOC_DAY")?.parse()?;
  let part: u16 = env::var("AOC_PART")?.parse()?;
  let mut buf = String::new();
  std::io::stdin().read_to_string(&mut buf)?;
  match (year, day, part) {
    (2020, 1, 1) => y2020::d01::p1(&buf),
    (2020, 1, 2) => y2020::d01::p2(&buf),
    (2020, 2, 1) => y2020::d02::p1(&buf),
    _ => return Err(SelectionError { year, day, part }.into()),
  }
  Ok(())
}

fn main() {
  match run() {
    Ok(()) => {}
    Err(e) => {
      eprintln!("error: {}", e);
      std::process::exit(1);
    }
  }
}
