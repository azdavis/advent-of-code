#!/bin/sh

set -eu

if [ "$#" -ne 1 ]; then
  echo "usage: $0 <year>"
  exit 1
fi

YEAR="$1"
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
pub fn p1(_: &str) -> u32 {
  todo!()
}

pub fn p2(_: &str) -> u32 {
  todo!()
}

#[test]
fn t() {
  let s = include_str!("input/d$DAY.txt");
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
  println!("{}", y$YEAR::d$DAY::p1(&inp));
}
EOF

"$EDITOR" "$SRC/input/d$DAY.txt" "$SRC/d$DAY.rs"
