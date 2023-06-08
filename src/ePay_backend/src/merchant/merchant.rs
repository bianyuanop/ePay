use std::collections::{BTreeMap, HashSet};
use std::time::{self, Duration};

use candid::{Principal, CandidType, Deserialize};
use ic_cdk_timers::TimerId;

use crate::management::state::MerchantConfig;
use crate::{types::Account};
use super::comment::Comment;
use super::order::Order;
use super::balance::Balance;

#[derive(CandidType, Deserialize)]
pub struct Merchant {
    pub owner: Principal,
    deposit_account: Account,
    balance: Balance,

    pub order_ptr: u64,
    orders: BTreeMap<u64, Order>,

    // depending on the implementation on the frontend 
    info_spec: Option<String>,
    info: Option<Vec<u8>>,

    orders_on_hold: Vec<u64>,
    pub blocked: bool,

    pub conf: MerchantConfig,
    pub comments: Vec<Comment>,
}

impl Default for Merchant {
    fn default() -> Self {
        Self { 
            owner: Principal::anonymous(),
            deposit_account: Account::from(Principal::anonymous()),
            balance: Balance::default(), 
            order_ptr: 0, 
            orders: BTreeMap::new(),
            info_spec: None,
            info: None,
            orders_on_hold: vec![],
            blocked: false,
            conf: MerchantConfig::default(),
            comments: vec![]
        }
    }
}

impl Merchant {
    pub fn new(conf: MerchantConfig) -> Self {
        Self { 
            owner: Principal::anonymous(),
            deposit_account: Account::from(Principal::anonymous()),
            balance: Balance::default(), 
            order_ptr: 0, 
            orders: BTreeMap::new(),
            info_spec: None,
            info: None,
            orders_on_hold: vec![],
            blocked: false,
            conf,
            comments: vec![]
        }
    }

    pub fn update_config(&mut self, conf: MerchantConfig) {
        self.conf = conf;
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

    pub fn check_orders_and_update(&mut self) {
        let mut left: Vec<u64> = vec![];
        let now = ic_cdk::api::time();
        for order_id in self.orders_on_hold.iter() {
            let order = self.orders.get_mut(order_id).unwrap();
            let order_created_at = order.timestamp;

            let time_elapsed = Duration::from_nanos(now - order_created_at);

            if time_elapsed >= Duration::from_secs(self.conf.order_on_hold_duration) && !order.is_controversial() {
                order.close();
            } else {
                left.push(*order_id);
            }
        }

        self.orders_on_hold = left;
    }
}

// used in manage canister 