#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   ./restore.sh <backup_file> [db_container_name]

BACKUP_FILE="$(pwd)/${1:-./backup.sql}"
SERVICE_NAME="${2:-${SERVICE_NAME:-myapp_db}}"
DB_USER="${DB_USER:-postgres}"

if [ -z "${BACKUP_FILE}" ]; then
  echo "Usage: $0 <backup_file> [db_container_name]"
  exit 1
fi

if [ ! -f "${BACKUP_FILE}" ]; then
  echo "Backup file not found: ${BACKUP_FILE}"
  exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
(
  cd "${PROJECT_ROOT}/releases/current/"

  if [ -n "${SERVICE_NAME}" ] && docker inspect "${SERVICE_NAME}" >/dev/null 2>&1; then
    echo "Restoring into container '${SERVICE_NAME}'..."

    docker compose exec -T "${SERVICE_NAME}" psql -U "${DB_USER}" < "${BACKUP_FILE}"
  else
    echo "No DB container found."
    exit 1
  fi
)

echo "Restore completed."

