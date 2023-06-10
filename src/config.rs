use candid::{Deserialize, CandidType};

#[derive(CandidType, Deserialize)]
pub struct Config {
    fee_rate: u8,
}