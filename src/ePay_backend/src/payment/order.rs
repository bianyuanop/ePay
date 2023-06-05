use std::{collections::HashMap, hash::Hash, vec};

use candid::{CandidType, Deserialize, Nat, Principal, parser::token};

use crate::{tokens::{TokenInfo}, types::Account};

use super::comment::Comment;

#[derive(CandidType, Deserialize, Clone)]
pub enum OrderStatus {
    Open,
    Controversial,

    // terminal status
    Unpaid,
    Refunded,
    Closed,
}


#[derive(CandidType, Deserialize, Clone)]
pub struct Order {
    id: u64,
    status: OrderStatus,
    // on chain time
    timestamp: u64,
    tokens_needed: HashMap<TokenInfo, Nat>,
    tokens_paid: HashMap<TokenInfo, Nat>,
    // token_info: Vec<>,
    // amount: Nat,
    payers: Vec<Account>,
    // payload is totally dependent on the implementation at the frontend
    payload: Option<Vec<u8>>,
    payload_spec: Option<String>,

    // used for controversial orders that may need admins to judge
    comments: Option<Vec<Comment>>
}

impl Order {
    pub fn generate_order(id: u64, timestamp: u64, tokens_needed: HashMap<TokenInfo, Nat>, payload: Option<Vec<u8>>, payload_spec: Option<String>) -> Self {
        let mut tokens_paid: HashMap<TokenInfo, Nat> = HashMap::new();
        for (k, v) in tokens_needed.iter() {
            tokens_paid.insert(k.clone(), Nat::from(0));
        }

        Self {
            id,
            status: OrderStatus::Open,
            timestamp,
            tokens_needed,
            tokens_paid,
            payers: vec![],
            payload,
            payload_spec,
            comments: None
        }
    }
}



pub struct OrderBrief {
    id: u64,
    // merchant:
}
