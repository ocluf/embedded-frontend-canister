# frontend canister

The standard asset canister does not lend itself well for DAO based control as permissions are not based on the canister controller model. That is why canisters that are controlled by the NNS like Internet Identity and the NNS frontend embed the frontend into the wasm binary instead of using an asset canister.

This repo contains a general canister to do that. Just put you frontend files in the "frontend" directory and run dfx build. Canister wasm binaries have a limit of 2MB so it's advisable to use a framework like preact or svelte which has a very small bundle size.
