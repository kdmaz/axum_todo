#!/usr/bin/env bash
if ! [ -x "$(command -v psql)" ]; then
	echo >&2 "Error: `psql` is not installed."
	echo >&2 "https://www.postgresql.org/download/"
	exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
	echo >&2 "Error: `sqlx` is not installed."
	echo >&2 "Use:"
	echo >&2 "cargo install sqlx-cli --no-default-features --features native-tls,postgres"
	echo >&2 "https://crates.io/crates/sqlx-cli"
	exit 1
fi

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=postgres}"
DB_NAME="${POSTGRES_DB:=todo_db}"
DB_PORT="${POSTGRES_PORT:=5444}"

docker run \
	-e POSTGRES_USER=${DB_USER} \
	-e POSTGRES_PASSWORD=${DB_PASSWORD} \
	-e POSTGRES_DB=${DB_NAME} \
	-p "${DB_PORT}":5432 \
	-d postgres \
	postgres -N 1000

export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
	sleep 1
done

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

>&2 echo "Postgres running on port ${DB_PORT}"