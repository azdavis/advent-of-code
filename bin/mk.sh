#!/bin/sh

set -eu

if [ "$#" -ne 2 ]; then
  echo "usage: $0 <year> <day>"
  exit 1
fi

YEAR="$1"
DAY="$2"

cd "$(dirname "$0")"
cd ..

mkdir -p "years/y$YEAR/src/input"

cat <<EOF > "years/y$YEAR/src/d$DAY.rs"
pub fn p1(_: &str) -> u32 {
  todo!()
}

pub fn p2(_: &str) -> u32 {
  todo!()
}
EOF

touch "years/y$YEAR/src/input/d$DAY.txt"

echo "pub mod d$DAY;" >> "years/y$YEAR/src/lib.rs"

cat <<EOF > runner/src/main.rs
fn main() {
  let inp = include_str!("../../years/y$YEAR/src/input/d$DAY.txt");
  println!("{}", y$YEAR::d$DAY::p1(&inp));
}
EOF

"$EDITOR" "years/y$YEAR/src/input/d$DAY.txt" "years/y$YEAR/src/d$DAY.rs"
