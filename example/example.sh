#!/bin/sh
cp ../lib/*.rs .
cargo run example.rstl example.rs
rustc -o example example.rs
./example myfile.txt
