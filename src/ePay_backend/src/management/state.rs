use std::collections::{HashSet, BTreeMap};


use candid::{Deserialize, CandidType, Principal};

use crate::tokens::TokenInfo;

#[derive(CandidType, Deserialize)]
pub struct StateInfo {
    owner: Principal,
    managers: HashSet<Principal>,
    token_allowed: HashSet<TokenInfo>,
}

impl StateInfo {
    pub fn add_manager(&mut self, manager: Principal) -> bool {
        self.managers.insert(manager)
    }

    pub fn remove_manager(&mut self, manager: Principal) -> bool {
        self.managers.remove(&manager)
    }

    pub fn is_manager(&self, manager: Principal) -> bool {
        self.managers.contains(&manager)
    }

    pub fn set_owner(&mut self, owner: Principal) -> bool {
        self.owner = owner;
        true
    }

    pub fn is_owner(&self, owner: Principal) -> bool {
        self.owner == owner
    }
}

impl Default for StateInfo {
    fn default() -> Self {
        Self { managers: HashSet::new(), owner: Principal::anonymous(), token_allowed: HashSet::new() }
    }
}

#[derive(CandidType, Deserialize, Default)]
pub struct MerchantDB {
    pub merchant_ptr: u64,
    merchants: BTreeMap<u64, Principal>,
}

impl MerchantDB {
    pub fn add_merchant(&mut self, merchant: Principal) {
        self.merchants.insert(self.merchant_ptr, merchant);
    }
}