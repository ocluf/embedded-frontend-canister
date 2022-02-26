use assets::{init_assets, AssetHashes, Assets, HttpRequest, HttpResponse};
use ic_kit::macros::*;

mod assets;

#[derive(Default)]
struct State {
    pub assets: Assets,
    pub asset_hashes: AssetHashes,
}

#[init]
fn init() {
    init_assets();
}

#[query]
fn http_request(req: HttpRequest) -> HttpResponse {
    return assets::http_request(req);
}
