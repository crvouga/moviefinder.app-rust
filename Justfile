check: #
  cargo check && cargo fmt

run: #
  just tw-build && npx kill-port 8000 && just watch

watch: #
  cargo watch -x run

fmt: #
  cargo fmt

db-start: #
  sudo docker-compose -f ./db/docker-compose.local.yml up -d

db-stop: #
  sudo docker-compose -f ./db/docker-compose.local.yml down

db-up: #
  npx dbmate -e DATABASE_URL --env-file ".env.local" up

db-down: #
  npx dbmate -e DATABASE_URL --env-file ".env.local" down

test: #
  TEST_ENV=int cargo test

cloc: #
  npx cloc src

tw4-setup-macos-x86: #
  curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v4.0.0-beta.1/tailwindcss-macos-x64 && chmod +x tailwindcss-macos-x64 && mv tailwindcss-macos-x64 tailwindcss

tw4-setup-macos-arm64: #
  curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v4.0.0-beta.1/tailwindcss-macos-arm64 && chmod +x tailwindcss-macos-arm64 && mv tailwindcss-macos-arm64 tailwindcss

tw-setup-macos-x86: #
  curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-macos-x64 && chmod +x tailwindcss-macos-x64 && mv tailwindcss-macos-x64 tailwindcss

tw-setup-macos-arm64: #
  curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-macos-arm64 && chmod +x tailwindcss-macos-arm64 && mv tailwindcss-macos-arm64 tailwindcss

tw: #
  ./tailwindcss -i ./public/input.css -o ./public/output.css --watch

tw-build: #
  ./tailwindcss -i ./public/input.css -o ./public/output.css --minify

preview: #
  docker build -t moviefinder-app . && docker run --rm --env-file .env moviefinder-app
