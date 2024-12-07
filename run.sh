#!/bin/bash
docker compose stop
docker compose start postgres
cp env .env
cargo build --release
./target/release/rustweb
