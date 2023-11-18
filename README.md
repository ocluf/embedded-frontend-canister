# Embedded Frontend Canister Starter Template

This repository offers a starter template for embedding frontend assets directly into a canister's WebAssembly (WASM) binary. It's a strategy similar to the one used by Internet Identity and NNS frontend, which are governed by the NNS DAO.

## Why Use This Template?

While DFINITY's SDK (`dfx`) simplifies frontend deployment using the [certified asset canister](https://github.com/dfinity/certified-assets), it's not the best fit for DAO-based management. The main issue is that you cannot query the internal list of principals authorized to update assets.

Our template provides a method similar to that employed by the NNS DAO projects. Here, frontend assets are integrated into the canister's binary itself, rather than being uploaded after installation.

## Considerations for Use

- **Canister Size Limit**: The current maximum canister size is 2MB, which is the max message size. We recommend using lightweight frontend frameworks like Svelte or Preact to keep the bundle size small. A Preact starter template is included in this example for your convenience.

- **Handling Larger Frontends**: If your frontend exceeds the size limit, you might want to serve non-critical assets (such as images) from a CDN or a separate canister.

This template is designed to make the setup process straightforward for projects that require DAO-compatible frontend deployment on the Internet Computer, offering a practical alternative to the standard certified asset canister approach.

(This information is probably outdated by now)
