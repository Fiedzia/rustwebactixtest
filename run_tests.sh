#!/bin/bash
docker compose stop
docker compose start postgres_test
cp env.test .env
diesel migration revert --all
diesel migration run
cargo test
