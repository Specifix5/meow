#!/bin/bash
export GIT_HASH=$(git rev-parse --short HEAD)
export GIT_HASH_LONG=$(git rev-parse HEAD)

echo "GIT_HASH: $GIT_HASH"
echo "GIT_HASH_LONG: $GIT_HASH_LONG"

cross build --target x86_64-unknown-linux-musl --release
