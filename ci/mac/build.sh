#!/bin/bash
set -eu

export PATH="~/.cargo/bin:$PATH"

cargo +stable-x86_64-apple-darwin build --release
