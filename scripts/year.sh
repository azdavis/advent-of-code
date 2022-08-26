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

mkdir -p "years/y$YEAR/src"

echo '#![deny(rust_2018_idioms)]' > "years/y$YEAR/src/lib.rs"
# extra blank line
echo > "years/y$YEAR/src/lib.rs"

cat <<EOF > "years/y$YEAR/Cargo.toml"
[package]
name = "y$YEAR"
version = "0.1.0"
edition = "2021"

[lib]
doctest = false

[dependencies]
helpers = { path = "../../helpers" }
EOF
