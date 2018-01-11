#!/bin/bash
set -eu

export PATH="~/.cargo/bin:$PATH"

if command -v rustfmt >/dev/null; then
    rustup run nightly-x86_64-apple-darwin rustfmt --version
else
    cargo +nightly-x86_64-apple-darwin  install rustfmt-nightly --force
fi

cargo +stable-x86_64-apple-darwin  fmt -- --write-mode diff
