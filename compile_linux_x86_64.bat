@echo off
for /f %%i in ('git rev-parse --short HEAD') do set GIT_HASH=%%i
for /f %%i in ('git rev-parse HEAD') do set GIT_HASH_LONG=%%i

start cmd /k "echo GIT_HASH: %GIT_HASH% & echo GIT_HASH_LONG: %GIT_HASH_LONG% & cross build --target x86_64-unknown-linux-musl --release"
