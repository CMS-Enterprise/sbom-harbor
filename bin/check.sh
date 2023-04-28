#!/bin/sh

# Stash unstaged changes to only check staged code
git stash -q --keep-index

# Run Cargo check on the staged Rust files
CHECK_RESULT=$(cargo check 2>&1)

if [ $? -ne 0 ]; then
  echo >&2 "error: cargo check found errors. Please fix them before committing."
  echo "$CHECK_RESULT" >&2
  git stash pop -q
  exit 1
else
  # Unstash unstaged changes
  git stash pop -q
fi

# Exit with success status
exit 0
