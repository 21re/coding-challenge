#!/bin/bash

set -e -u -o pipefail

name=coding-challenge

rm -rf target
docker build --build-arg TOOLCHAIN=nightly rust-musl-builder -t 21re-rust-musl-builder
docker run --rm -it -v "$(pwd)":/home/rust/src 21re-rust-musl-builder cargo build --release

docker build . -t 21re/coding-challenge



