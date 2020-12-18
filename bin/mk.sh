#!/bin/sh

set -eu

if [ "$#" -ne 2 ]; then
  echo "usage: $0 <year> <day>"
  exit 1
fi

YEAR="$1"
DAY="$2"
SRC="years/y$YEAR/src"

cd "$(dirname "$0")"
cd ..

mkdir -p "$SRC/input"

cat <<EOF > "$SRC/d$DAY.rs"
pub fn p1(s: &str) -> u32 {
  todo!()
}

pub fn p2(s: &str) -> u32 {
  todo!()
}

#[test]
fn t() {
  let inp = include_str!("input/d$DAY.txt");
  // assert_eq!(p1(inp), ___);
  // assert_eq!(p2(inp), ___);
}
EOF

touch "$SRC/input/d$DAY.txt"

echo "pub mod d$DAY;" >> "$SRC/lib.rs"
sort -o "$SRC/lib.rs" "$SRC/lib.rs"

cat <<EOF > runner/src/main.rs
fn main() {
  let inp = include_str!("../../$SRC/input/d$DAY.txt");
  println!("{}", y$YEAR::d$DAY::p1(&inp));
}
EOF

"$EDITOR" "$SRC/input/d$DAY.txt" "$SRC/d$DAY.rs"
