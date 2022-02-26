#!/usr/bin/env bash
echo "Skipping build as requested"

pushd frontend || exit
npm run build
popd || exit

pushd frontend/build || exit
tar cJv --exclude .last_build_id -f ../../assets.tar.xz .
popd || exit.sh



ic-cdk-optimizer target/wasm32-unknown-unknown/release/frontend_canister.wasm -o target/wasm32-unknown-unknown/release/frontend_canister.wasm