#!/bin/bash
set -e

# Database configuration
POSTGRES_USER="${POSTGRES_USER:-moviefinder-app}"
POSTGRES_PASSWORD="${POSTGRES_PASSWORD:-moviefinder-app}"
POSTGRES_DB="${POSTGRES_DB:-moviefinder-app}"
POSTGRES_DATA_DIR="/var/lib/postgresql/16/main"
POSTGRES_LOG="/var/lib/postgresql/postgres.log"

echo "Starting PostgreSQL initialization..."

# Override DATABASE_URL from .env file - we'll set it to point to local PostgreSQL
unset DATABASE_URL

# Create data directory parent if needed
mkdir -p /var/lib/postgresql
chown -R postgres:postgres /var/lib/postgresql 2>/dev/null || true

# Initialize PostgreSQL data directory if it doesn't exist or is incomplete
if [ ! -d "$POSTGRES_DATA_DIR" ] || [ -z "$(ls -A $POSTGRES_DATA_DIR 2>/dev/null)" ] || [ ! -f "$POSTGRES_DATA_DIR/postgresql.conf" ]; then
    echo "Initializing PostgreSQL data directory..."
    # Remove incomplete directory if it exists
    if [ -d "$POSTGRES_DATA_DIR" ]; then
        echo "Removing incomplete data directory..."
        rm -rf "$POSTGRES_DATA_DIR"
    fi
    mkdir -p "$POSTGRES_DATA_DIR"
    chown -R postgres:postgres "$POSTGRES_DATA_DIR"
    
    # Initialize database cluster
    sudo -u postgres /usr/lib/postgresql/16/bin/initdb -D "$POSTGRES_DATA_DIR" --auth-local=trust --auth-host=scram-sha-256
    
    # Configure PostgreSQL
    echo "host all all 0.0.0.0/0 scram-sha-256" >> "$POSTGRES_DATA_DIR/pg_hba.conf"
    echo "listen_addresses='*'" >> "$POSTGRES_DATA_DIR/postgresql.conf"
    echo "ssl = off" >> "$POSTGRES_DATA_DIR/postgresql.conf"
    
    echo "PostgreSQL data directory initialized"
fi

# Ensure log file directory is writable
mkdir -p "$(dirname "$POSTGRES_LOG")"
chown -R postgres:postgres "$(dirname "$POSTGRES_LOG")" 2>/dev/null || true
touch "$POSTGRES_LOG" 2>/dev/null || true
chown postgres:postgres "$POSTGRES_LOG" 2>/dev/null || true

# Start PostgreSQL in the background using postgres binary directly
echo "Starting PostgreSQL server..."
sudo -u postgres /usr/lib/postgresql/16/bin/postgres -D "$POSTGRES_DATA_DIR" > "$POSTGRES_LOG" 2>&1 &
POSTGRES_PID=$!

# Give PostgreSQL a moment to start
sleep 2

# Check if PostgreSQL process is still running
if ! kill -0 $POSTGRES_PID 2>/dev/null; then
    echo "PostgreSQL process died immediately. Checking log:"
    if [ -f "$POSTGRES_LOG" ]; then
        echo "=== PostgreSQL log file ==="
        cat "$POSTGRES_LOG"
        echo "=== End of log file ==="
    else
        echo "Log file not found at $POSTGRES_LOG"
    fi
    echo "Checking PostgreSQL data directory:"
    ls -la "$POSTGRES_DATA_DIR" || true
    echo "Checking if postgres user exists:"
    id postgres || true
    exit 1
fi

# Wait for PostgreSQL to be ready
echo "Waiting for PostgreSQL to be ready..."
for i in {1..30}; do
    if sudo -u postgres /usr/lib/postgresql/16/bin/pg_isready -U postgres > /dev/null 2>&1; then
        echo "PostgreSQL is ready"
        break
    fi
    if [ $i -eq 30 ]; then
        echo "PostgreSQL failed to start after 30 seconds"
        echo "PostgreSQL log:"
        if [ -f "$POSTGRES_LOG" ]; then
            cat "$POSTGRES_LOG"
        fi
        exit 1
    fi
    echo "PostgreSQL is not ready yet, waiting... ($i/30)"
    sleep 1
done

# Create user and database if they don't exist
echo "Setting up database user and database..."
sudo -u postgres /usr/lib/postgresql/16/bin/psql -U postgres <<-EOSQL
    DO \$\$
    BEGIN
        IF NOT EXISTS (SELECT FROM pg_catalog.pg_user WHERE usename = '$POSTGRES_USER') THEN
            CREATE USER "$POSTGRES_USER" WITH PASSWORD '$POSTGRES_PASSWORD';
        END IF;
    END
    \$\$;
    
    SELECT 'CREATE DATABASE "$POSTGRES_DB"'
    WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = '$POSTGRES_DB')\gexec
    
    GRANT ALL PRIVILEGES ON DATABASE "$POSTGRES_DB" TO "$POSTGRES_USER";
EOSQL

# Grant permissions on the public schema
echo "Granting permissions on public schema..."
sudo -u postgres /usr/lib/postgresql/16/bin/psql -U postgres -d "$POSTGRES_DB" <<-EOSQL
    GRANT ALL ON SCHEMA public TO "$POSTGRES_USER";
    ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO "$POSTGRES_USER";
    ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON SEQUENCES TO "$POSTGRES_USER";
EOSQL

echo "Database user and database setup complete"

# Set DATABASE_URL environment variable (with SSL disabled for local development)
export DATABASE_URL="postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:5432/${POSTGRES_DB}?sslmode=disable"

echo "DATABASE_URL set to: $DATABASE_URL"

# Update .env file in the container to have the correct DATABASE_URL
# This ensures that when the application loads .env, it gets the correct value
if [ -f "/app/.env" ]; then
    echo "Updating .env file with correct DATABASE_URL..."
    # Use awk to safely replace any line that starts with DATABASE_URL= (with or without quotes, with or without spaces)
    # This preserves all other lines exactly as they are
    awk -v new_db_url="DATABASE_URL=\"postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:5432/${POSTGRES_DB}?sslmode=disable\"" '
        /^[[:space:]]*DATABASE_URL[[:space:]]*=/ { 
            print new_db_url
            found=1
            next
        }
        { print }
        END {
            if (!found) {
                print new_db_url
            }
        }
    ' /app/.env > /app/.env.tmp && mv /app/.env.tmp /app/.env
    echo "Updated .env file with DATABASE_URL pointing to localhost"
fi

# Run database migrations
echo "Running database migrations..."
dbmate --url "$DATABASE_URL" up
echo "Database migrations completed successfully"

# Start the application
echo "Starting application..."
exec "$@"
