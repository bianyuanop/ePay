use std::{collections::{HashSet, BTreeMap}, time::Duration};


use candid::{Deserialize, CandidType, Principal};

use crate::{tokens::TokenInfo};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MerchantConfig {
    // secs
    pub order_check_duration: u64,
    pub order_on_hold_duration: u64,
    pub fee_rate: f32,
    pub fee_to: Principal,
    pub token_allowed: HashSet<TokenInfo>,
}

impl Default for MerchantConfig {
    fn default() -> Self {
        Self {
            // 1 hour to check
            order_check_duration: 60,
            // hold 7 days before merchant can redeem the assets paid by buyers
            order_on_hold_duration: 60,
            fee_rate: 0.001,
            fee_to: Principal::anonymous(),
            token_allowed: HashSet::new()
        }
    }
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct StateInfo {
    pub owner: Principal,
    pub managers: HashSet<Principal>,
    pub merchant_conf: MerchantConfig,
    pub fee_to: Principal,
    pub user_canister: Option<Principal>
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

    pub fn set_order_check_duration(&mut self, duration_in_secs: u64) {
        self.merchant_conf.order_check_duration = duration_in_secs;
    }

    pub fn set_order_on_hold_duration(&mut self, duration_in_secs: u64) {
        self.merchant_conf.order_on_hold_duration = duration_in_secs;
    }
}

impl Default for StateInfo {
    fn default() -> Self {
        Self { 
            managers: HashSet::new(),
            owner: Principal::anonymous(), 
            merchant_conf: MerchantConfig::default(),
            fee_to: Principal::anonymous(),
            user_canister: None
        }
    }
}

#[derive(CandidType, Deserialize, Default)]
pub struct MerchantDB {
    pub merchant_ptr: u64,
    pub merchants: BTreeMap<u64, Principal>,
}

impl MerchantDB {
    pub fn add_merchant(&mut self, merchant: Principal) -> u64 {
        self.merchants.insert(self.merchant_ptr, merchant);
        self.merchant_ptr += 1;

        self.merchant_ptr-1
    }

    pub fn update_merchant(&mut self, id: u64, merchant: Principal) {
        self.merchants.insert(id, merchant);
    }

    pub fn get_merchant_principal(&self, id: u64) -> Option<Principal> {
        match self.merchants.get(&id) {
            Some(m) => {
                Some((*m).clone())
            },
            None => None
        }
    }
}