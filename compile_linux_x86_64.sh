export GIT_HASH=$(git rev-parse --short HEAD)
export GIT_HASH_LONG=$(git rev-parse HEAD)
cross build --target x86_64-unknown-linux-musl --release