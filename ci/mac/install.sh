#!/bin/bash
set -eu

export PATH="~/.cargo/bin:$PATH"

if command -v rustup >/dev/null; then
   rustup install stable-x86_64-apple-darwin 
else
    curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable-x86_64-apple-darwin  -y
fi

rustup install nightly-x86_64-apple-darwin
