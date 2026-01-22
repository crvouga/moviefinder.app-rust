FROM rust:latest

WORKDIR /app

# Install system dependencies
RUN echo "Installing system dependencies..." \
    && apt-get update \
    && apt-get install -y curl \
    && echo "System dependencies installed"

# Install dbmate
RUN echo "Installing dbmate..." \
    && curl -fsSL -o /usr/local/bin/dbmate https://github.com/amacneil/dbmate/releases/latest/download/dbmate-linux-amd64 \
    && chmod +x /usr/local/bin/dbmate \
    && echo "dbmate installed successfully"

# Copy source code
RUN echo "Copying source code..."
COPY . .

# Build the Rust application
RUN echo "Building Rust application..." \
    && cargo build --release \
    && echo "Rust application built successfully"

# The application will handle migrations at startup
CMD ./target/release/moviefinder-app
