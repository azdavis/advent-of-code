#!/bin/sh

set -eu

if [ "$#" -gt 1 ]; then
  echo "usage: $0 [<year>]"
  exit 1
fi

if [ "$#" -eq 0 ]; then
  YEAR="$(date +%Y)"
else
  YEAR="$1"
fi

SRC="years/y$YEAR/src"

cd "$(dirname "$0")"
cd ..

mkdir -p "$SRC/input"

i=1
for f in "$SRC"/d*.rs; do
  if [ -f "$f" ]; then
    i=$((i + 1))
  fi
done
DAY="$(printf '%02d' "$i")"

cat <<EOF > "$SRC/d$DAY.rs"
pub fn p1(s: &str) -> usize {
  s.len()
}

pub fn p2(s: &str) -> usize {
  s.len()
}

#[test]
fn t() {
  // let s = include_str!("input/d$DAY.txt");
  // assert_eq!(p1(s), ___);
  // assert_eq!(p2(s), ___);
}
EOF

touch "$SRC/input/d$DAY.txt"

echo "pub mod d$DAY;" >> "$SRC/lib.rs"
rustfmt "$SRC/lib.rs"

cat <<EOF > runner/src/main.rs
fn main() {
  let s = include_str!("../../$SRC/input/d$DAY.txt");
  println!("p1: {}", y$YEAR::d$DAY::p1(s));
  println!("p2: {}", y$YEAR::d$DAY::p2(s));
}
EOF

"$EDITOR" "$SRC/input/d$DAY.txt" "$SRC/d$DAY.rs"
