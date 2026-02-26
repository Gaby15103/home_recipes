#!/bin/sh
set -e

echo "Running database migrations..."
/usr/local/bin/migration-tool up -u "$DATABASE_URL"

echo "Starting the application..."
# Use 'exec' so the app becomes PID 1
exec /usr/local/bin/app