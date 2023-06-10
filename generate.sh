#!/bin/sh
cargo build --release
target/release/rt > test.ppm

