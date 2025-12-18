#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

ARCHIVE="$PROJECT_ROOT/bootstrap.tar.gz"

(
  cd "${PROJECT_ROOT}"

  echo "Creating bootstrap package: $ARCHIVE"
  tar czf "$ARCHIVE" -C "$PROJECT_ROOT" deploy/db deploy/scripts
)

echo "Done."

