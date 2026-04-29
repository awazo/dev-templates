#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   ./migrate.sh [--clear-db] [app_name] [db_service_name]

# デフォルト
CLEAR_DB=false
APP_NAME="${APP_NAME:-myapp}"
DB_SERVICE_NAME="${DB_SERVICE_NAME:-myapp_db}"

# 位置引数の一時的な保存用配列
POSITIONAL=()

for arg in "$@"; do
  case "$arg" in
    --clear-db)
      CLEAR_DB=true
      ;;
    -*)
      echo "Unknown option: $arg"
      exit 1
      ;;
    *)
      POSITIONAL+=("$arg")
      ;;
  esac
done

# 位置引数を展開
if [[ ${#POSITIONAL[@]} -ge 1 ]]; then
  APP_NAME="${POSITIONAL[0]}"
fi

if [[ ${#POSITIONAL[@]} -ge 2 ]]; then
  DB_SERVICE_NAME="${POSITIONAL[1]}"
fi

if [[ ${#POSITIONAL[@]} -gt 2 ]]; then
  echo "Too many arguments"
  exit 1
fi


SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
(
  cd "${PROJECT_ROOT}/releases/current/"

  echo "Starting deploy..."
  echo "CLEAR_DB=${CLEAR_DB}"

  if [ "$CLEAR_DB" = true ]; then
    echo "[Reset] Stopping and removing containers + volumes..."
    docker compose down --volumes
  fi

  echo "[Step 1/3] Ensure DB is running..."
  docker compose up -d "${DB_SERVICE_NAME}"

  echo "[Step 2/3] Run migrations..."
  docker compose run --rm "${APP_NAME}" migrate

  echo "[Step 3/3] Start / update app..."
  docker compose up -d "${APP_NAME}"
)

echo "Finished."
