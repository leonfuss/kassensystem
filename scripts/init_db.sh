#!/bin/bash
set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error psql is not installed."
    exit 1 
fi 

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    echo >&2 "User: "
    echo >&2 "  cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres"
    echo >&2 "to install it."
    exit 1
fi 

# check for custom user, otherwise use default ("postgres")
DB_USER="${POSTGRES_USER:=postgres}"

# check for custom password, otherwise use default ("password")
DB_PASSWORT="${POSTGRES_PASSWORD:=password}"

# check for custom database name, otherwise use default ("kassensystem")
DB_NAME="${POSTGRES_DB:=kassensystem}"

# check fo custom db port, otherwise use default ("5432")
DB_PORT="${POSTGRES_PORT:=5432}"

# lauch postgres using docker
# skips if postgres docker instance is already running
if [[ -z "${SKIP_DOCKER}" ]]
then
    docker run \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORT} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}":5432 \
        -d postgres \
        postgres -N 1000 # increase max connection number for testing
fi

# Keepp pinging postgres until it is ready to accept commands
export PGPASSWORD="${DB_PASSWORT}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do 
    >&2 echo "Postgres ist still unavialable - sleeping"
    sleep 1 
done

>&2 echo "Postgres is up and running on port ${DB_PORT}! - running migrations now"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORT}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

>&2 echo "Postgres hat been migrated, ready to go!"
