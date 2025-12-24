#!/bin/bash
set -e

# Environment setup
export LEPTOS_SITE_ADDR="${LEPTOS_SITE_ADDR:-0.0.0.0:3000}"
export LEPTOS_SITE_ROOT="${LEPTOS_SITE_ROOT:-site}"
export API_PORT="${API_PORT:-3001}"
export API_URL="http://127.0.0.1:${API_PORT}"

echo "=== XFTradesmen Starting ==="
echo "LEPTOS_SITE_ADDR: $LEPTOS_SITE_ADDR"
echo "API_URL: $API_URL"

# Start Backend API in background (if it exists)
if [ -f "/app/api" ]; then
    echo "Starting Backend API on port $API_PORT..."
    PORT=$API_PORT /app/api &
    BACKEND_PID=$!
    echo "Backend PID: $BACKEND_PID"
    sleep 2
else
    echo "No API binary found, skipping backend..."
fi

# Start Frontend SSR in foreground
echo "Starting Frontend on $LEPTOS_SITE_ADDR..."
exec /app/frontend-leptos
