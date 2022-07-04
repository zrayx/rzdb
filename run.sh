#!/bin/bash

cargo fmt
cargo clippy &&
#RUST_BACKTRACE=1 cargo run --example data_types
RUST_BACKTRACE=1 cargo test
echo --------------------------------------------------------------------------------
inotifywait -q -e close_write examples src Cargo.toml run.sh
clear

exec ./run.sh