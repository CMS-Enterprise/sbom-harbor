#!/bin/sh

# Check if Clippy is installed
if ! command -v cargo-clippy >/dev/null 2>&1; then
  echo >&2 "error: Clippy not installed. Please install it using 'rustup component add clippy'"
  exit 1
fi

# Stash unstaged changes to only check and fix staged code
git stash -q --keep-index

# Run Clippy with auto-fix on the staged Rust files
cargo clippy --fix -Zunstable-options --allow-dirty --allow-staged -- -D warnings

# Stage the fixed files
git add .

# Unstash unstaged changes
git stash pop -q

# Exit with success status
exit 0
