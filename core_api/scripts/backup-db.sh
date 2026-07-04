#!/bin/bash

set -e

TIMESTAMP=$(date +"%Y-%m-%d_%H-%M-%S")
BACKUP_FILE="backups/backup_$TIMESTAMP.dump"

echo "📦 Criando backup em $BACKUP_FILE..."

docker exec -t rust_server_postgres \
  pg_dump -U $POSTGRES_USER -d $POSTGRES_DB -F c -f /tmp/backup.dump

docker cp rust_server_postgres:/tmp/backup.dump "$BACKUP_FILE"

docker exec -t rust_server_postgres rm /tmp/backup.dump

echo "✅ Backup concluído!"