# embedded frontend canister

dfx contains built-in support for using the [certified asset canister](https://github.com/dfinity/certified-assets). This canister makes it very easy to upload a frontend but it doesn't lend itself well for DAO based control. For instance it keeps an internal list of principals that can update the assets but you can't query this list of principals. 

This is probably why you see a different method used for the Internet Identity and the NNS frontend, both of which are controlled by the NNS DAO. In these canisters the frontend assets are baked in the canister wasm binary itself instead of uploading the assets after installing. This repo provides a starter template to do that as well.       

Note that the size of a canister at the moment is limited to 2MB (the max message size). Using this method it is advisable to use a frontend framework with a small bundle size like svelte or preact (preact starter template is included in this example).  If your canister is bigger you can consider loading non essential images through a CDN or a different canister. 
