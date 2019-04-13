#!/bin/sh

set -ex

for dir in $(ls)
do
	if ! [ -d $dir ]
	then
		continue
	fi

	(
		cd $dir
		cargo build
		target/debug/$dir &
		pid=$!
		sleep 10
		kill $pid
	)
done
