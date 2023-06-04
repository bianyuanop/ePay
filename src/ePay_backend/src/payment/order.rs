use candid::{CandidType, Deserialize, Nat, Principal};

use crate::{tokens::{TokenInfo}, types::Account};

use super::comment::Comment;

#[derive(CandidType, Deserialize)]
pub enum OrderStatus {
    Open,
    Controversial,

    // terminal status
    Unpaid,
    Refunded,
    Closed,
}

#[derive(CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrderIndex(u64);

#[derive(CandidType, Deserialize)]
pub struct Order {
    id: OrderIndex,
    status: OrderStatus,
    timestamp: i64,
    token_info: TokenInfo,
    amount: Nat,
    payee: Account,
    payers: Vec<Account>,
    // payload is totally dependent on the implementation at the frontend
    payload: Option<Vec<u8>>,
    payload_spec: Option<String>,

    // used for controversial orders that may need admins to judge
    comments: Option<Vec<Comment>>
}

pub struct OrderBrief {
    id: OrderIndex,
}