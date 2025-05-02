#!/usr/bin/env bash

set -euo pipefail

USERNAME=$1
BACKEND_PORT=$2

echo "Starting up hypostats..."
echo -e "\tUsername: ${USERNAME}"
echo -e "\tPort: ${BACKEND_PORT}"

# start database
if lsof -Pi :BACKEND_PORT -sTCP:LISTEN -t >/dev/null ; then
  echo "Starting up database..."
  # start database
  cargo pgrx run
  echo "Database started."
    while ! nc -z localhost "${BACKEND_PORT}"; do
    sleep 1
    done
    echo "quit"
else
  echo "Database already running."
fi

echo "Starting up backend..."
# start backend app
cargo run --bin hypostats -- "${USERNAME}" "${BACKEND_PORT}" &
echo "Backend started."

echo "Starting up frontend..."

# Start frontend app
cd frontend-app/
npm start

cd -