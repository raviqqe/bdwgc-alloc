#!/bin/sh

set -ex

cd $(dirname $0)

for cargo_file in */Cargo.toml; do
  cargo run --bin $(dirname $cargo_file) &
  pid=$!
  sleep 20
  kill $pid
done
