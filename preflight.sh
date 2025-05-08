#!/bin/bash

cargo build --verbose 
echo -n "Press Enter to continue"
read e

cargo test --features  --verbose

echo -n "Press Enter to continue"
read e

cargo test --doc --verbose

echo -n "Press Enter to continue"
read e

cargo fmt --all -- --check

echo -n "Press Enter to continue"
read e

cargo clippy --all-targets  -- -D warnings

