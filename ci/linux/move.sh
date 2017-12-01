#!/bin/bash
set -eu

export PATH="~/.cargo/bin:$PATH"

mv ../../target/release/j2y ../../target/release/j2y-linux
