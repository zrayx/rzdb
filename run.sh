#!/bin/bash

cargo fmt
cargo clippy &&
RUST_BACKTRACE=1 cargo run --example basic
echo --------------------------------------------------------------------------------
inotifywait -q -e close_write examples src Cargo.toml run.sh
clear

exec ./run.sh