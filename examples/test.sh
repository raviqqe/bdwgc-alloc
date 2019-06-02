#!/bin/sh

set -ex

cargo build

for cargo_file in */Cargo.toml
do
	target/debug/$(dirname $cargo_file) &
	pid=$!
	sleep 10
	kill $pid
done
