use candid::{CandidType, Deserialize, Nat, Principal};

#[derive(CandidType, Deserialize, Clone)]
pub struct Comment {
    issuer: Principal,
    payload_spec: String,
    payload: Vec<u8>,
    timestamp: u64,
}

impl Comment {
    pub fn new(issuer: Principal, payload: Vec<u8>, payload_spec: String) -> Self {
        Self {
            issuer,
            payload,
            payload_spec,
            timestamp: ic_cdk::api::time(),
        }
    }
}