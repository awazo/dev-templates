#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   ./release-unpack.sh [app_name] [version]

APP_NAME="${1:-${APP_NAME:-myapp}}"
VERSION="${2:-${VERSION:-tmp}}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
(
  cd "${PROJECT_ROOT}/releases/"

  echo "unpacking release from ${APP_NAME}-${VERSION}.tar.gz..."
  mkdir "${VERSION}"
  tar xzf "${APP_NAME}"-"${VERSION}".tar.gz -C "${VERSION}"
)

echo "Finished."

