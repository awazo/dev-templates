#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   ./db-backup.sh [output_dir] [db_service_name]

SERVICE_NAME="${2:-${SERVICE_NAME:-myapp_db}}"
DB_USER="${DB_USER:-postgres}"
DB_NAME="${DB_NAME:-app}"

OUTDIR="${1:-./backups}"
TIMESTAMP="$(date +'%Y-%m-%d_%H%M%S')"
OUTFILE="db_${TIMESTAMP}.sql"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
(
  cd "${PROJECT_ROOT}"

  mkdir -p "${OUTDIR}"
  OUTPATH="$(cd "${OUTDIR}" && pwd)"

  cd "${PROJECT_ROOT}/releases/current/"

  if [ -n "${SERVICE_NAME}" ] && docker inspect "${SERVICE_NAME}" >/dev/null 2>&1; then
    echo "Detected docker container '${SERVICE_NAME}'. Running pg_dump inside container..."
    docker compose exec -T "${SERVICE_NAME}" pg_dump -U "${DB_USER}" -cC --column-inserts --if-exists "${DB_NAME}" > "${OUTPATH}/${OUTFILE}"
    echo "Backup saved to: ${OUTPATH}/${OUTFILE}"
  else
    echo "No DB container found."
    exit 1
  fi
)

echo "Finished."

