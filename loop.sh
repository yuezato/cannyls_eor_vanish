#!/bin/bash

set -eux

# prog=./target/debug/cannyls_eor_vanish
prog=./target/release/cannyls_eor_vanish

rm -f test.lusf

for i in `seq 1 5000`; do
    echo $i
    $prog --seed $i
    if ! ((i % 15)); then
	rm -f test.lusf
    fi
done
