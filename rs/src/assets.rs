use ic_certified_map::{labeled, labeled_hash, AsHashTree, Hash, RbTree};
use ic_kit::candid::CandidType;
use ic_kit::ic;
use mime_guess;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use sha2::{Digest, Sha256};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::Read;

use crate::State;

type HeaderField = (String, String);

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpRequest {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: ByteBuf,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpResponse {
    status_code: u16,
    headers: Vec<HeaderField>,
    body: ByteBuf,
}

const LABEL_ASSETS: &[u8] = b"http_assets";

#[derive(Default)]
pub struct AssetHashes(RbTree<Vec<u8>, Hash>);

impl From<&Assets> for AssetHashes {
    fn from(assets: &Assets) -> Self {
        let mut asset_hashes = Self::default();
        for (path, asset) in assets.0.iter() {
            asset_hashes
                .0
                .insert(path.as_bytes().to_vec(), hash_bytes(&asset.bytes));
        }
        asset_hashes
    }
}

/// An asset to be served via HTTP requests.
#[derive(CandidType, Clone, Deserialize, PartialEq, Debug)]
pub struct Asset {
    headers: Vec<HeaderField>,
    bytes: Vec<u8>,
}

impl Asset {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            headers: vec![],
            bytes,
        }
    }
}

#[derive(Default, CandidType, Deserialize, PartialEq, Debug)]
pub struct Assets(HashMap<String, Asset>);

impl Assets {
    fn insert<S: Into<String>>(&mut self, path: S, asset: Asset) {
        self.0.insert(path.into(), asset);
    }

    fn get(&self, path: &str) -> Option<&Asset> {
        self.0.get(path)
    }
}

pub fn http_request(req: HttpRequest) -> HttpResponse {
    let parts: Vec<&str> = req.url.split('?').collect();
    match parts[0] {
        request_path => {
            let s = ic::get::<State>();
            let certificate_header =
                make_asset_certificate_header(&s.asset_hashes.borrow(), request_path);
            match s.assets.get(request_path) {
                Some(asset) => {
                    let mut headers = asset.headers.clone();
                    headers.push(certificate_header);
                    let mime_guess = mime_guess::from_path(request_path);
                    if let Some(content_type) = mime_guess.first_raw() {
                        if request_path.ends_with("map") {
                            headers
                                .push(("Content-Type".to_string(), "application/json".to_string()));
                        } else {
                            headers.push(("Content-Type".to_string(), content_type.to_string()));
                        }
                    }
                    HttpResponse {
                        status_code: 200,
                        headers,
                        body: ByteBuf::from(asset.bytes.clone()),
                    }
                }
                None => HttpResponse {
                    status_code: 404,
                    headers: vec![],
                    body: ByteBuf::from(format!("Asset {} not found.", request_path)),
                },
            }
        }
    }
}

fn make_asset_certificate_header(asset_hashes: &AssetHashes, asset_name: &str) -> (String, String) {
    let certificate = ic::data_certificate().unwrap_or_else(|| {
        ic::trap("data certificate is only available in query calls");
    });

    let witness = asset_hashes.0.witness(asset_name.as_bytes());
    let tree = labeled(LABEL_ASSETS, witness);
    let mut serializer = serde_cbor::ser::Serializer::new(vec![]);
    serializer.self_describe().unwrap();
    tree.serialize(&mut serializer)
        .unwrap_or_else(|e| ic::trap(&format!("failed to serialize a hash tree: {}", e)));
    (
        "IC-Certificate".to_string(),
        format!(
            "certificate=:{}:, tree=:{}:",
            base64::encode(&certificate),
            base64::encode(&serializer.into_inner())
        ),
    )
}

pub fn hash_bytes(value: impl AsRef<[u8]>) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(value.as_ref());
    hasher.finalize().into()
}

/// Insert an asset into the state.
pub fn insert_asset<S: Into<String> + Clone>(path: S, asset: Asset) {
    ic::print(format!("Inserting asset {}", &path.clone().into()));
    let state = ic::get_mut::<State>();
    let path = path.into();

    let index = "index.html";
    if path.split('/').last() == Some(index) {
        // Add the directory, with trailing slash, as an alternative path.
        // Note: Without the trailing slash the location of "." is the parent, so breaks resource links.
        let prefix_len = path.len() - index.len();
        let dirname = &path[..prefix_len];
        state
            .asset_hashes
            .0
            .insert(dirname.as_bytes().to_vec(), hash_bytes(&asset.bytes));
        state.assets.insert(dirname, asset.clone());
    }

    state
        .asset_hashes
        .0
        .insert(path.as_bytes().to_vec(), hash_bytes(&asset.bytes));
    state.assets.insert(path, asset);

    update_root_hash(&state.asset_hashes);
}

// used both in init and post_upgrade
pub fn init_assets() {
    let compressed = include_bytes!("../../assets.tar.xz").to_vec();
    let mut decompressed = Vec::new();
    lzma_rs::xz_decompress(&mut compressed.as_ref(), &mut decompressed).unwrap();
    let mut tar: tar::Archive<&[u8]> = tar::Archive::new(decompressed.as_ref());
    for entry in tar.entries().unwrap() {
        let mut entry = entry.unwrap();

        if !entry.header().entry_type().is_file() {
            continue;
        }

        let name_bytes = entry
            .path_bytes()
            .into_owned()
            .strip_prefix(b".")
            .unwrap()
            .to_vec();

        let name = String::from_utf8(name_bytes.clone()).unwrap_or_else(|e| {
            ic::trap(&format!(
                "non-utf8 file name {}: {}",
                String::from_utf8_lossy(&name_bytes),
                e
            ));
        });

        let mut bytes = Vec::new();
        entry.read_to_end(&mut bytes).unwrap();
        ic::print(format!("{}: {}", &name, bytes.len()));

        insert_asset(name, Asset::new(bytes));
    }
}

fn update_root_hash(a: &AssetHashes) {
    let prefixed_root_hash = &labeled_hash(LABEL_ASSETS, &a.0.root_hash());
    ic::set_certified_data(&prefixed_root_hash[..]);
}
