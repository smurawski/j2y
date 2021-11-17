#! /usr/bin/env bash

read -p "Locally with JSON"
cargo run -- -s JSON test.json output.yml

echo ""
read -p "Locally with YAML"
cargo run -- -s YAML test.yml output.json