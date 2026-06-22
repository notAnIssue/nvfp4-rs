#!/usr/bin/env bash

set -euo pipefail


echo "── fmt ────────────────────────────────────────"
cargo fmt --all -- --check                                  

echo "── clippy ─────────────────────────────────────"
cargo clippy --workspace --all-targets -- -D warnings       

echo "── test ───────────────────────────────────────"
cargo test --workspace                                      

echo "── doc ────────────────────────────────────────"
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps


echo "── private-fingerprint check ──────────────────"

! git grep -nE "colibri|assets/notes" -- '*.md' '*.rs' '*.py' '*.toml' '*.yml'


# Phase 5
#   ruff check python/
#   mypy python/nvfp4_rs/
#   pytest python/tests
# Phase 7
#   cargo test -p nvfp4-cuda --features cuda

echo "preflight OK"