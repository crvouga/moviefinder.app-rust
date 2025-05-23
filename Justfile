check: #
  clear && cargo check && cargo fmt

run: #
  clear && just kp && export STAGE=local && just watch

kp: #
  npx kill-port 3000

ngrok: #
  ngrok http 3000 --subdomain moviefinder

watch: #
  cargo watch -x run

fmt: #
  cargo fmt

db: #
  export $(grep -v '^#' .env | xargs) && psql "$DATABASE_URL"

db-start: #
  sudo docker-compose -f ./db/docker-compose.local.yml up -d

db-stop: #
  sudo docker-compose -f ./db/docker-compose.local.yml down

db-up: #
  npx dbmate -e DATABASE_URL up

db-down: #
  npx dbmate -e DATABASE_URL down

db-new: #
  npx dbmate -e DATABASE_URL new ${name}

test: #
  just test-unit
  
test-unit: #
  clear && STAGE=test TEST_ENV=unit cargo test

test-int: #
  clear && STAGE=test TEST_ENV=int cargo test

cloc: #
  npx cloc src

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
