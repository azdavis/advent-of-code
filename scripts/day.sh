#!/bin/sh

set -eu

if [ "$#" -gt 1 ]; then
  echo "usage: $0 [<year>]"
  exit 1
fi

need_cmd() {
  if ! command -v "$1" > /dev/null; then
    echo "need $1: check $2"
    exit 1
  fi
}

need_cmd fix-ws 'https://github.com/azdavis/fix-ws.git'
need_cmd cargo 'https://rustup.rs'
need_cmd rustfmt 'https://rustup.rs'
need_cmd curl 'https://curl.se'

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
pub fn p1(_: &str) -> u32 {
  0
}

pub fn p2(_: &str) -> u32 {
  0
}

#[test]
fn t() {
  let s = include_str!("input/d$DAY.txt");
  assert_eq!(p1(s), 0);
  assert_eq!(p2(s), 0);
}
EOF

SESSION="$(cat session.txt)"
curl --cookie "session=$SESSION" \
  "https://adventofcode.com/$YEAR/day/$i/input" \
  > "$SRC/input/d$DAY.txt"

fix-ws "$SRC/input/d$DAY.txt"

echo "pub mod d$DAY;" >> "$SRC/lib.rs"
rustfmt "$SRC/lib.rs"

cat <<EOF > run.sh
#!/bin/sh
set -eux
cargo test --release -p y$YEAR -- --nocapture --color=always d$DAY
EOF
chmod +x run.sh

"$EDITOR" "$SRC/input/d$DAY.txt" "$SRC/d$DAY.rs"
