#!/bin/bash

set -e

if [ -z "$1" ]; then
  echo "❌ Uso: ./scripts/restore-db.sh <arquivo.dump>"
  exit 1
fi

BACKUP_FILE=$1

echo "♻️ Restaurando backup $BACKUP_FILE..."

docker cp "$BACKUP_FILE" rust_server_postgres:/tmp/restore.dump

docker exec -t rust_server_postgres \
  pg_restore -U $POSTGRES_USER -d $POSTGRES_DB --clean --if-exists /tmp/restore.dump

docker exec -t rust_server_postgres rm /tmp/restore.dump

echo "✅ Restore concluído!"