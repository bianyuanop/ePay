use candid::{CandidType, Deserialize, Nat, Principal};

use crate::{tokens::{TokenInfo}, types::Account};

pub enum OrderStatus {
    Open,
    Controversial,

    // terminal status
    Unpaid,
    Refunded,
    Closed,
}

pub struct Order {
    key: u64,
    page: usize,
    status: OrderStatus,
    timestamp: i64,
    token_info: TokenInfo,
    amount: Nat,
    merchant: Account,
    payee: Account,
    // payload is totally dependent on the implementation at the frontend
    payload: Option<Vec<u8>>,
    payload_spec: Option<String>,
}

impl Order {
}