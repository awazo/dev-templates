#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   ./release-pack.sh [app_name]

APP_NAME="${1:-${APP_NAME:-myapp}}"
VERSION="$(date +"%Y-%m-%d_%H%M%S")"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
(
  cd "${PROJECT_ROOT}"

  echo "Building release..."
  cargo build --release --workspace

  echo "Copy target/release to deploy/releases/app"
  cp "${PROJECT_ROOT}/target/release/${APP_NAME}" "${PROJECT_ROOT}/deploy/releases/app/."
  cp "${PROJECT_ROOT}/.env.example" "${PROJECT_ROOT}/deploy/releases/."

  echo "packing release to ${APP_NAME}-${VERSION}.tar.gz..."
  cd "${PROJECT_ROOT}/deploy/releases/"
  tar czf "${APP_NAME}"-"${VERSION}".tar.gz docker-compose.yml docker/ app/ .env.example
)

echo "Finished."

