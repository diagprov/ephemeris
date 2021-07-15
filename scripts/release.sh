#!/bin/sh

cargo build --target=x86_64-unknown-linux-musl --release
mkdir dist
cp target/x86_64-unknown-linux-musl/release/ephemeris dist/ephemeris
