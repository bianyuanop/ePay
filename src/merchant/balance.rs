use candid::{Deserialize, CandidType, Nat, Principal, parser::token};
use std::{collections::HashMap};

use crate::tokens::TokenInfo;

#[derive(CandidType, Deserialize, Clone)]
pub struct TokenBalance {
    token_info: TokenInfo,
    balance: Nat
}

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct Balance {
    // token name -> balance
    token_balances: HashMap<Principal, TokenBalance>,    
}

impl Balance {
    pub fn add(&mut self, token_info: &TokenInfo, amount: &Nat) {
        let token_balance = self.token_balances.get_mut(&token_info.principal);
        match token_balance {
            Some(b) => {
                b.balance += (*amount).clone();
            },
            None => {
                self.token_balances.insert(token_info.principal, TokenBalance { token_info: *token_info, balance: (*amount).clone() });
            }
        }
    }
}