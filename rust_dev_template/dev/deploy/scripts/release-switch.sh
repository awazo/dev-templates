#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   ./release-switch.sh [target_version]

VERSION="${1:-${VERSION:-tmp}}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
(
  cd "${PROJECT_ROOT}/releases/"

  echo "switch release version to ${VERSION}..."
  ln -sfn "${VERSION}" ./current
)

echo "Finished."

