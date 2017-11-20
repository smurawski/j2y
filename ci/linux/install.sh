#!/bin/bash
set -eu

export PATH="~/.cargo/bin:$PATH"

if command -v rustup >/dev/null; then
   rustup install stable-x86_64-unknown-linux-gnu
else
    curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable-x86_64-unknown-linux-gnu -y
fi

