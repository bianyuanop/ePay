use std::collections::{BTreeMap, HashSet, HashMap};
use std::ops::{Mul, Sub, Div};
use std::time::{self, Duration};

use candid::{Principal, CandidType, Deserialize, Nat};
use ic_cdk_timers::TimerId;

use crate::management::state::MerchantConfig;
use crate::{types::Account};
use super::comment::Comment;
use super::order::Order;
use super::balance::Balance;

#[derive(CandidType, Deserialize, Clone)]
pub struct Merchant {
    pub id: u64,
    pub owner: Principal,
    pub deposit_account: Account,
    pub balance: Balance,
    pub fee: Balance,

    pub order_ptr: u64,
    pub orders: BTreeMap<u64, Order>,

    // depending on the implementation on the frontend 
    pub info_spec: Option<String>,
    pub info: Option<Vec<u8>>,

    pub orders_on_hold: Vec<u64>,
    pub blocked: bool,

    pub conf: MerchantConfig,
    pub comments: Vec<Comment>,
    pub verified: bool,
}

impl Default for Merchant {
    fn default() -> Self {
        Self { 
            id: 0,
            owner: Principal::anonymous(),
            deposit_account: Account::from(Principal::anonymous()),
            balance: Balance::default(), 
            fee: Balance::default(),
            order_ptr: 0, 
            orders: BTreeMap::new(),
            info_spec: None,
            info: None,
            orders_on_hold: vec![],
            blocked: false,
            conf: MerchantConfig::default(),
            comments: vec![],
            verified: false
        }
    }
}

impl Merchant {
    pub fn new(conf: MerchantConfig) -> Self {
        Self { 
            id: 0,
            owner: Principal::anonymous(),
            deposit_account: Account::from(Principal::anonymous()),
            balance: Balance::default(), 
            fee: Balance::default(),
            order_ptr: 0, 
            orders: BTreeMap::new(),
            info_spec: None,
            info: None,
            orders_on_hold: vec![],
            blocked: false,
            conf,
            comments: vec![],
            verified: false
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

    pub fn calculate_fee(fee_rate: f32, amount: &Nat) -> (Nat, Nat) {
        // TODO: safe math here
        ((*amount).clone(), Nat::from(0))
    }

    pub fn check_orders_and_update(&mut self) {
        let mut left: Vec<u64> = vec![];
        let now = ic_cdk::api::time();
        for order_id in self.orders_on_hold.iter() {
            let order = self.orders.get_mut(order_id).unwrap();
            let order_created_at = order.timestamp;

            let time_elapsed = Duration::from_nanos(now - order_created_at);

            if time_elapsed >= Duration::from_secs(self.conf.order_on_hold_duration) && !order.is_controversial() {
                // fee application
                for (token_info, amount) in order.tokens_needed.iter() {
                    let (to_merchant, to_network) = Merchant::calculate_fee(self.conf.fee_rate, amount);
                    self.balance.add(token_info, &to_merchant);
                    self.fee.add(token_info, &to_network);
                }
                order.close();
            } else {
                left.push(*order_id);
            }
        }

        self.orders_on_hold = left;
    }

    pub fn get_merchant_masked_off_orders(&self) -> Merchant {
        // TODO: should do with default -> fill to prevent high consumption of copying `orders` as there will be thousands of orders
        let mut res = (*self).clone();
        res.orders_on_hold = vec![];
        res.orders = BTreeMap::new();

        res
    }
}

// used in manage canister 