#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
(
  cd "$PROJECT_ROOT"

  echo "==> Running cargo fmt..."
  cargo fmt --all -- --check

  echo "==> Running cargo clippy..."
  cargo clippy --workspace --all-targets --all-features -- -D warnings

  echo "==> Running tests..."
  cargo test --workspace
)
echo "==> All checks passed."

