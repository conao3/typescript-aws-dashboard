#!/usr/bin/env bash
set -euo pipefail

echo "waiting for postgres to be ready..."
until PGPASSWORD="${POSTGRES_PASSWORD}" psql -h postgres -U dashboard -d dashboard -c '\q' 2>/dev/null; do
  sleep 1
done

echo "applying database schema..."
PGPASSWORD="${POSTGRES_PASSWORD}" psqldef -U dashboard -h postgres dashboard --file=/app/schema.sql

echo "starting backend server..."
exec /app/backend
