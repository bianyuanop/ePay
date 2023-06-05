use candid::{CandidType, Deserialize, Nat, Principal};

#[derive(CandidType, Deserialize, Clone)]
pub struct Comment {
    issuer: Principal,
    payload_spec: String,
    payload: Vec<u8>,
}