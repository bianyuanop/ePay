use std::collections::BTreeMap;

use candid::{Principal, CandidType, Deserialize};

use crate::{types::Account, user::balance::Balance, payment::order::Order};

#[derive(CandidType, Deserialize)]
pub struct Merchant {
    pub owner: Principal,
    pub deposit_account: Account,
    balance: Balance,

    pub order_ptr: u64,
    orders: BTreeMap<u64, Order>,

    // depending on the implementation on the frontend 
    info_spec: Option<String>,
    info: Option<Vec<u8>>,
}

impl Merchant {
    pub fn new() -> Self {
        Self { 
            owner: Principal::anonymous(),
            deposit_account: Account::from(Principal::anonymous()),
            balance: Balance::default(), 
            order_ptr: 0, 
            orders: BTreeMap::new(),
            info_spec: None,
            info: None
        }
    }

    pub fn add_order(&mut self, order: Order) -> u64 {
        self.orders.insert(self.order_ptr, order);
        self.order_ptr += 1;

        self.order_ptr-1
    }

    pub fn get_order_mut(&mut self, order_id: u64) -> Option<&mut Order> {
        self.orders.get_mut(&order_id)
    }

    pub fn get_order(&self, order_id: u64) -> Option<&Order> {
        self.orders.get(&order_id)
    }
}

// used in manage canister 
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


