#!/bin/bash
rm -rf cvudp cvudp.zip cvudp.h
cargo clean
cargo build --release
mkdir cvudp
mv target/release/libdoorbell.a cvudp
mv cvudp.h cvudp
7z a cvudp.zip cvudp
