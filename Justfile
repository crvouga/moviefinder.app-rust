check: #
  cargo check

run: #
  npx kill-port 8000 && npx concurrently "just tw-watch" "just watch"

watch: #
  cargo watch -x run

db-start: #
  sudo docker-compose -f ./docker-compose.dev.yml up -d

db-stop: #
  sudo docker-compose -f ./docker-compose.dev.yml down

db-up: #
  npx dbmate -e DATABASE_URL up

db-down: #
  npx dbmate -e DATABASE_URL down

test: #
  TEST_ENV=int cargo test

cloc: #
  npx cloc src

tw-setup-macos-x86: #
  curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-macos-x64 && chmod +x tailwindcss-macos-x64 && mv tailwindcss-macos-x64 tailwindcss

tw-setup-macos-arm64: #
  curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-macos-arm64 && chmod +x tailwindcss-macos-arm64 && mv tailwindcss-macos-arm64 tailwindcss

tw-watch: #
  ./tailwindcss -i ./public/input.css -o ./public/output.css --watch

tw-build: #
  ./tailwindcss -i ./public/input.css -o ./public/output.css --minify

preview: #
  docker build -t moviefinder-app . && docker run --rm --env-file .env moviefinder-app
