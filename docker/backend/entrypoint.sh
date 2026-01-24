#!/bin/sh
set -e

echo "? Waiting for Postgres..."

until diesel database setup >/dev/null 2>&1; do
  echo "Postgres not ready yet, retrying..."
  sleep 2
done

echo "? Postgres is ready"
if [ "$(diesel migration list | grep 2026_01_21_add_seeds | wc -l)" -eq 0 ]; then
  echo "📦 Running Diesel migrations..."
  diesel migration run
fi

echo "🚀 Starting backend..."
exec "$@"