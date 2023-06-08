use candid::{Deserialize, CandidType, Nat};
use std::{collections::HashMap};

use crate::tokens::TokenInfo;

#[derive(CandidType, Deserialize)]
pub struct TokenBalance {
    token_info: TokenInfo,
    balance: Nat
}

#[derive(CandidType, Deserialize, Default)]
pub struct Balance {
    // token name -> balance
    token_balances: HashMap<String, TokenBalance>,    
}