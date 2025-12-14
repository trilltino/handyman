#!/bin/bash
set -e

# Defaults
export API_PORT=${API_PORT:-3001}
export LEPTOS_SITE_ADDR=${LEPTOS_SITE_ADDR:-"0.0.0.0:3000"}
export API_URL="http://127.0.0.1:${API_PORT}"

echo "Starting XFHandyman..."

# Wait for DB
if [ -n "$DB_HOST" ]; then
    echo "Waiting for database at $DB_HOST..."
    while ! pg_isready -h $DB_HOST -p $DB_PORT -U $DB_USER; do
        sleep 1
    done
fi

# Run Migrations
echo "Running migrations..."
./api --migrate

# Start Backend API in background
echo "Starting Backend API on port $API_PORT..."
# Override PORT for the backend process
PORT=$API_PORT ./api &
BACKEND_PID=$!

# Wait briefly for backend to initialize
sleep 2

# Start Frontend SSR in foreground
echo "Starting Frontend SSR on $LEPTOS_SITE_ADDR..."
# API_URL env var is already set for the frontend to proxy to
./frontend-leptos &
FRONTEND_PID=$!

# Wait for any process to exit
wait -n

# Exit with status of process that exited first
exit $?
