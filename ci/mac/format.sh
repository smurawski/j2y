#!/bin/bash
set -eu

export PATH="~/.cargo/bin:$PATH"

if command -v rustfmt >/dev/null; then
    rustup run stable-x86_64-apple-darwin  rustfmt --version
else
    cargo +stable-x86_64-apple-darwin  install rustfmt --force
fi

cargo +stable-x86_64-apple-darwin  fmt -- --write-mode diff
