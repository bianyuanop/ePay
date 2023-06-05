use candid::{Nat, CandidType, Deserialize};
use ic_ledger_types::Timestamp;

use crate::{tokens::TokenInfo, types::Account};

#[derive(CandidType, Deserialize, Clone)]
pub struct Transaction {
    related_order: Option<u64>,
    token: TokenInfo,
    amount: Nat,
    payer: Account,
    payee: Account,
    timestamp: Timestamp,
    finished: bool,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct TransactionDB {
    finished: Vec<Transaction>,
    processing: Vec<Transaction>,
}