#!/usr/bin/env bash
set -euo pipefail

echo "buildng frontend"
pushd frontend || exit
npm run build
popd || exit

echo "creating compressed bundle"
pushd frontend/build || exit
tar cJv --exclude .last_build_id -f ../../assets.tar.xz .
popd || exit.sh

echo "building wasm binary"
cargo build --target wasm32-unknown-unknown --release

ic-cdk-optimizer target/wasm32-unknown-unknown/release/frontend_canister.wasm -o ./frontend_canister.wasm