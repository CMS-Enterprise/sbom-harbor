#!/bin/sh

# Check if rustfmt is installed
if ! command -v rustfmt >/dev/null 2>&1; then
  echo >&2 "error: rustfmt not installed. Please install it using 'rustup component add rustfmt'"
  exit 1
fi

# Stash unstaged changes to only format staged code
git stash -q --keep-index

# Format staged Rust files using rustfmt
STAGED_RUST_FILES=$(git diff --cached --name-only --diff-filter=ACMR -- '*.rs')

if [ -n "$STAGED_RUST_FILES" ]; then
  for FILE in $STAGED_RUST_FILES; do
    rustfmt $FILE
    git add $FILE
  done
fi

# Unstash unstaged changes
git stash pop -q

# Exit with success status
exit 0
