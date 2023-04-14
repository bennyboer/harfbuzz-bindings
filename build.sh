#!/usr/bin/env bash

cargo install cargo-vcpkg
cargo vcpkg build
cargo build --release