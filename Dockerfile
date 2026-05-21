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

# Install Tailwind CSS standalone CLI (pinned to last v3 release; v4 changed
# config format and would break tailwind.config.js).
RUN echo "Installing Tailwind CSS CLI..." \
    && ARCH=$(dpkg --print-architecture) \
    && case "$ARCH" in \
         amd64) TW_ARCH="x64" ;; \
         arm64) TW_ARCH="arm64" ;; \
         *) echo "Unsupported architecture: $ARCH" && exit 1 ;; \
       esac \
    && curl -fsSL -o /usr/local/bin/tailwindcss \
        "https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.17/tailwindcss-linux-${TW_ARCH}" \
    && chmod +x /usr/local/bin/tailwindcss \
    && echo "Tailwind CSS CLI installed"

# Copy source code
RUN echo "Copying source code..."
COPY . .

# Copy and set up entrypoint script
COPY docker-entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

# Compile Tailwind once production source is in place. Output is served as a
# static asset by the app at GET /output.css.
RUN echo "Compiling Tailwind CSS..." \
    && tailwindcss -i ./public/input.css -o ./public/output.css --minify \
    && echo "Tailwind CSS compiled"

# Build the Rust application
RUN echo "Building Rust application..." \
    && cargo build --release \
    && echo "Rust application built successfully"

# Use entrypoint script to initialize PostgreSQL, run migrations, and start the app
ENTRYPOINT ["docker-entrypoint.sh"]
CMD ["./target/release/moviefinder-app"]
