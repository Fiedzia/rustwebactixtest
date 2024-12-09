#!/bin/bash
docker compose stop postgres
docker compose start postgres_test
sleep 2
cp env.test .env
diesel migration revert --all
diesel migration run
cargo test --  --nocapture
