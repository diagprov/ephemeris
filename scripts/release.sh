#!/bin/sh

cargo build -p ephemeris --target=x86_64-unknown-linux-musl --release
cargo build -p ephemeris-cmd --target=x86_64-unknown-linux-musl --release
mkdir -p dist/x86_64/
cp -v target/x86_64-unknown-linux-musl/release/ephemeris dist/x86_64/ephemeris
