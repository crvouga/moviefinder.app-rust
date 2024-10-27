run: #
  cargo fmt && cargo clippy && cargo run

db-start: #
  sudo docker-compose -f ./docker-compose.local.yml up -d

db-stop: #
  sudo docker-compose -f ./docker-compose.local.yml down

db-up: #
  npx dbmate -e DATABASE_URL up

db-down: #
  npx dbmate -e DATABASE_URL down

test: #
  TEST_ENV=int cargo test

