#!/usr/bin/env bash
set -e

echo "[INFO] Waiting for database..."

until sqlx migrate info --source /app/migrations; do
  echo "[INFO] Database not ready yet..."
  sleep 2
done

echo "[INFO] Running migrations..."
sqlx migrate run --source /app/migrations

echo "[INFO] Starting API..."
exec /app/rust_raw_server