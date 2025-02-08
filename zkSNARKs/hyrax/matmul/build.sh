#!/bin/sh

for i in 16 32 64 128; do
    echo $i
done | python3 matmult.py