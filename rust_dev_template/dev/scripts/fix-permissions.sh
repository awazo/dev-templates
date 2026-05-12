#!/usr/bin/env bash
# fix-permissions.sh
#
# /workspace 以下のファイルのパーミッションを rw-rw-r-- (664) に修正するスクリプト。
# VS Code のファイル作成操作が umask 0022 で動くため、
# グループの書き込み権限が付かない問題を手動で修正するために使用する。
#
# 使い方:
#   bash scripts/fix-permissions.sh           # /workspace 全体を対象
#   bash scripts/fix-permissions.sh src/      # 特定のディレクトリを対象
#   bash scripts/fix-permissions.sh src/main.rs  # 特定のファイルを対象

set -euo pipefail

TARGET="${1:-/workspace}"

if [[ ! -e "$TARGET" ]]; then
    echo "Error: not found: $TARGET" >&2
    exit 1
fi

echo "target: $TARGET"

# fix file permissions to 664
find "$TARGET" -type f ! -perm /g+w -exec chmod g+w {} +

echo "Finished."
