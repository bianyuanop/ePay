use candid::{CandidType, Deserialize, Nat, Principal};

#[derive(CandidType, Deserialize)]
pub struct Comment {
    issuer: Principal,
    payload_spec: String,
    payload: Vec<u8>,
}