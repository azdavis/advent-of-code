use std::io::Read as _;

fn run() -> Result<(), Box<dyn std::error::Error>> {
  let mut inp = String::new();
  std::io::stdin().read_to_string(&mut inp)?;
  println!("{}", y2020::d07::p2(&inp));
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
