use std::collections::HashMap;

use candid::{Nat, Principal};

use crate::{types::Account, tokens::TokenInfo};

pub struct Split {
    pub tokens_needed: HashMap<TokenInfo, Nat>,
    pub payers: HashMap<Account, Vec<(TokenInfo, Nat)>>,

    pub related_order: u64,
}

impl Split {
    
}