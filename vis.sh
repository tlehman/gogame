#!/bin/sh
rm ./target/debug/gogame
cargo build
./target/debug/gogame | neato -Tpng | imgcat
