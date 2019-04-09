#!/bin/bash

set -eux

# prog=./target/debug/miss_eor
prog=./target/release/miss_eor

rm -f test.lusf

for i in `seq 1 5000`; do
    echo $i
    $prog
    if ! ((i % 10)); then
	rm -f test.lusf
    fi
done
