#!/bin/sh

set -ex

cd $(dirname $0)

cargo build

for cargo_file in */Cargo.toml; do
  target/debug/$(dirname $cargo_file) &
  pid=$!
  sleep 20
  kill $pid
done
