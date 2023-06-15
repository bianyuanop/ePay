use std::{collections::HashMap, hash::Hash, vec, fmt::format, ops::Sub};

use candid::{CandidType, Deserialize, Nat, Principal, parser::token};

use crate::{tokens::{TokenInfo, TokenType, icrc1::{ICRC1, TransferArgs}, dip20::DIP20}, types::Account, utils::order_id_to_subaccount};

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
    pub id: u64,
    pub status: OrderStatus,
    // on chain time
    pub timestamp: u64,
    pub tokens_needed: HashMap<TokenInfo, Nat>,
    // receiving account seems redundant
    pub receiving_account: Account,
    // can be extended by a token scheme contract on behalf payers
    pub payer: Account,

    pub payload: Option<Vec<u8>>,
    pub payload_spec: Option<String>,

    pub paid: bool,

    // used for controversial orders that may need admins to judge
    pub comments: Vec<Comment>,
}

impl Order {
    pub fn generate_order(id: u64, timestamp: u64, tokens_needed: HashMap<TokenInfo, Nat>, payload: Option<Vec<u8>>, payload_spec: Option<String>, merchant_canister: Principal, payer: Account) -> Self {
        Self {
            id,
            status: OrderStatus::Open,
            timestamp,
            tokens_needed,
            // tokens_paid,
            receiving_account: Account { owner: merchant_canister, subaccount: Some(order_id_to_subaccount(id)) },
            // payers: vec![],
            payer,
            // payment_scheme: PaymentScheme::single_payer(payer, &tokens_needed),
            payload,
            payload_spec,
            comments: vec![],
            paid: false
        }
    }

    pub fn mark_as_paid(&mut self) {
        self.paid = true;
    }

    pub fn mark_as_refunded(&mut self) {
        self.status = OrderStatus::Refunded;
    }

    pub fn insert_comment(&mut self, issuer: Principal, payload: Vec<u8>, payload_spec: String) {
        if self.status != OrderStatus::Controversial { self.status = OrderStatus::Controversial }
        self.comments.push(Comment::new(issuer, payload, payload_spec));
    }

    pub async fn pay(&self) -> Result<bool, String> {
        for (token_info, amount) in self.tokens_needed.iter() {
            match token_info.token_type {
                TokenType::DIP20 => {
                    let token = DIP20::new(token_info.principal);
                    let receipt = token.transfer_from(self.payer.owner, self.receiving_account.owner, (*amount).clone()).await;
                    match receipt {
                        Ok(_) =>  (),
                        Err(e) => return Err(format!("{:?}", e).into())
                    }
                },
                _ => return Err(format!("unsupported token").into())
            }
        }
        Ok(true)
    }

    pub async fn refund(&self) -> Result<bool, String> {
        for (token_info, amount) in self.tokens_needed.iter() {
            match token_info.token_type {
                TokenType::DIP20 => {
                    let token = DIP20::new(token_info.principal);
                    let metadata = token.get_metadata().await;
                    let fee = metadata.fee;
                    let mut amount2refund = (*amount).clone();
                    // the implementation of DIP20 requires `balance > amount + fee`
                    amount2refund = amount2refund.sub(fee).sub(Nat::from(1));

                    let receipt = token.transfer(self.payer.owner, amount2refund).await;
                    match receipt {
                        Ok(_) =>  (),
                        Err(e) => return Err(format!("{:?}", e).into())
                    }
                },
                _ => return Err(format!("unsupported token").into())
            }
        }
        Ok(true)
    }

    pub fn close(&mut self) {
        self.status = OrderStatus::Closed;
    }

    pub fn is_controversial(&self) -> bool {
        self.status == OrderStatus::Controversial
    }
}