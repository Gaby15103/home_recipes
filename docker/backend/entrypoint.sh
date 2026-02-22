#!/bin/sh
set -e

echo "🔍 Checking Database connection..."
# A simple way to wait for Postgres without Diesel CLI:
until nc -z postgres 5432; do
  echo "Postgres is unavailable - sleeping"
  sleep 2
done

echo "✅ Postgres is up!"

# If you have a migration sub-command in your main app:
# ./backend migrate up

echo "🚀 Starting backend..."
exec "$@"