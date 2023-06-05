use std::{collections::HashMap, hash::Hash, vec};

use candid::{CandidType, Deserialize, Nat, Principal, parser::token};

use crate::{tokens::{TokenInfo, TokenType, icrc1::{ICRC1, TransferArgs}}, types::Account, merchant::transaction::Transaction, utils::order_id_to_subaccount};

use super::{comment::Comment, merchant};

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub enum OrderStatus {
    Open,
    Controversial,

    // terminal status
    Unpaid,
    Refunded,
    Closed,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum OrderType {
    Instant,
    PreSet,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Order {
    id: u64,
    pub status: OrderStatus,
    // on chain time
    timestamp: u64,
    tokens_needed: HashMap<TokenInfo, Nat>,
    // tokens_paid: HashMap<TokenInfo, Nat>,
    receiving_account: Account,
    // token_info: Vec<>,
    // amount: Nat,
    // payers: Vec<Account>,
    // tokens_needed_per_payer: Vec<Token>
    // payload is totally dependent on the implementation at the frontend
    payload: Option<Vec<u8>>,
    payload_spec: Option<String>,

    // used for controversial orders that may need admins to judge
    comments: Vec<Comment>,
}

impl Order {
    pub fn generate_order(id: u64, timestamp: u64, tokens_needed: HashMap<TokenInfo, Nat>, payload: Option<Vec<u8>>, payload_spec: Option<String>, merchant_canister: Principal) -> Self {
        Self {
            id,
            status: OrderStatus::Open,
            timestamp,
            tokens_needed,
            // tokens_paid,
            receiving_account: Account { owner: merchant_canister, subaccount: Some(order_id_to_subaccount(id)) },
            // payers: vec![],
            payload,
            payload_spec,
            comments: vec![],
        }
    }

    pub async fn is_paid(&self) -> bool {
        let mut paid = true;
        for (token_info, amount) in self.tokens_needed.iter() {
            paid &= match token_info.token_type {
                TokenType::ICRC1 => {
                    let token = ICRC1::new(token_info.principal);
                    if token.balance_of(&self.receiving_account).await >= *amount {
                        true
                    } else {
                        false
                    }
                },
                _ => false
            };
        }
        paid
    }

    pub fn mark_as_paid(&mut self) {
        self.status = OrderStatus::Closed;
    }

    pub fn insert_comment(&mut self, issuer: Principal, payload: Vec<u8>, payload_spec: String) {
        if self.status != OrderStatus::Controversial { self.status = OrderStatus::Controversial }
        self.comments.push(Comment::new(issuer, payload, payload_spec));
    }
}



#[derive(CandidType, Deserialize)]
pub struct OrderBrief {
    id: u64,
    merchant_canister_principal: Principal,
}
