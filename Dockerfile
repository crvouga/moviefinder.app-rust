FROM rust:latest

WORKDIR /app

RUN apt-get update \
    && apt-get install -y curl \
    && curl -fsSL -o /usr/local/bin/dbmate https://github.com/amacneil/dbmate/releases/latest/download/dbmate-linux-amd64 \
    && chmod +x /usr/local/bin/dbmate
# && curl -fsSL -o /usr/local/bin/tailwindcss https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 \
# && chmod +x /usr/local/bin/tailwindcss

COPY . .

# RUN tailwindcss -i ./public/input.css -o ./public/output.css --minify
# ARG DATABASE_URL
# CMD dbmate --url $DATABASE_URL up && ./target/release/moviefinder-app

RUN cargo build --release


CMD ./target/release/moviefinder-app
