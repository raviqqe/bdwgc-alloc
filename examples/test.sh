#!/bin/sh

set -ex

cd $(dirname $0)

cargo build

for cargo_file in */Cargo.toml; do
  cargo run --bin $(dirname $cargo_file)
done
