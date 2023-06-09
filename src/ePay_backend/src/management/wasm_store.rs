use candid::CandidType;
use serde_derive::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct WasmStorage {
    pub merchant_wasm: Option<Vec<u8>>,
    pub user_wasm: Option<Vec<u8>>,
}

impl Default for WasmStorage {
    fn default() -> Self {
        Self {
            merchant_wasm: None,
            user_wasm: None
        }
    }
}