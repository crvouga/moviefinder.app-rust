FROM rust:latest

WORKDIR /app

# Install system dependencies including PostgreSQL
RUN echo "Installing system dependencies..." \
    && apt-get update \
    && apt-get install -y curl gnupg lsb-release sudo \
    && mkdir -p /usr/share/keyrings \
    && curl -fsSL https://www.postgresql.org/media/keys/ACCC4CF8.asc | gpg --dearmor -o /usr/share/keyrings/postgresql.gpg \
    && echo "deb [signed-by=/usr/share/keyrings/postgresql.gpg] http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list \
    && apt-get update \
    && apt-get install -y postgresql-16 postgresql-client-16 \
    && echo "postgres ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers \
    && echo "System dependencies installed"

# Install dbmate
RUN echo "Installing dbmate..." \
    && curl -fsSL -o /usr/local/bin/dbmate https://github.com/amacneil/dbmate/releases/latest/download/dbmate-linux-amd64 \
    && chmod +x /usr/local/bin/dbmate \
    && echo "dbmate installed successfully"

# Copy source code
RUN echo "Copying source code..."
COPY . .

# Copy and set up entrypoint script
COPY docker-entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

# Build the Rust application
RUN echo "Building Rust application..." \
    && cargo build --release \
    && echo "Rust application built successfully"

# Use entrypoint script to initialize PostgreSQL, run migrations, and start the app
ENTRYPOINT ["docker-entrypoint.sh"]
CMD ["./target/release/moviefinder-app"]
