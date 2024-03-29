use std::borrow::Borrow;
use std::collections::{BTreeMap, HashSet, HashMap};
use std::ops::{Mul, Sub, Div, Add};
use std::result;
use std::time::{self, Duration};

use candid::{Principal, CandidType, Deserialize, Nat};
use ic_cdk_timers::TimerId;

use crate::management::state::MerchantConfig;
use crate::tokens::dip20::DIP20;
use crate::tokens::TokenType;
use crate::{types::Account};
use super::comment::Comment;
use super::notify::{self, Notifier};
use super::order::Order;
use super::balance::{Balance, TokenBalance};

pub type DepositLog = Vec<Result<Nat, String>>;

#[derive(CandidType, Deserialize, Clone)]
pub struct DepositResult {
    pub log: DepositLog,
    pub new_balance: Balance
}

impl Default for DepositResult {
    fn default() -> Self {
        Self {
            log: vec![], 
            new_balance: Balance::default()
        }
    }
}

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
    pub notifer: Option<Notifier>
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
            verified: false,
            notifer: None
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
            verified: false,
            notifer: None
        }
    }

    pub fn update_config(&mut self, conf: MerchantConfig) {
        self.conf = conf;
    }

    pub fn add_order(&mut self, order: Order) -> u64 {
        self.orders.insert(self.order_ptr, order);
        self.order_ptr += 1;

        // hold order
        self.orders_on_hold.push(self.order_ptr-1);

        self.order_ptr-1
    }

    pub fn get_order_mut(&mut self, order_id: u64) -> Option<&mut Order> {
        self.orders.get_mut(&order_id)
    }

    pub fn get_order(&self, order_id: u64) -> Option<&Order> {
        self.orders.get(&order_id)
    }

    #[allow(unused_variables)]
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
                if order.paid {
                    // fee application
                    for (token_info, amount) in order.tokens_needed.iter() {
                        let (to_merchant, to_network) = Merchant::calculate_fee(self.conf.fee_rate, amount);
                        self.balance.add(token_info, &to_merchant);
                        self.fee.add(token_info, &to_network);
                    }
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

    pub async fn deposit(&self) -> DepositResult {
        let mut log: DepositLog = vec![];
        let mut new_balance = Balance::default();

        for (token_principal, balance) in self.balance.token_balances.iter() {
            let token_info = balance.token_info;
            match token_info.token_type {
                TokenType::DIP20 => {
                    let token = DIP20::new(*token_principal);
                    let metadata = token.get_metadata().await;
                    let fee = metadata.fee;
                    let amount2deposit = DIP20::calculate_transferable(balance.balance.clone(), fee);

                    let transfer_res = token.transfer(self.deposit_account.owner, amount2deposit).await;

                    match transfer_res {
                        Ok(n) => {
                            log.push(Ok(n));
                            // limit of dip20 token, can only transfer `balance - 1` tokens
                            let token_balance = TokenBalance {
                                token_info: token_info.clone(),
                                balance: Nat::from(1)
                            };
                            new_balance.token_balances.insert(token_info.principal, token_balance);
                        },
                        Err(e) => {
                            new_balance.token_balances.insert(token_info.principal, (*balance).clone());
                            log.push(Err(format!("{:?}", e).into()));
                        }
                    }
                },
                _ => {
                    ic_cdk::println!("undepositable");
                    log.push(Err(format!("{} is not supported, this looks like a internal error", token_info.principal).into()))
                }
            }
        }

        DepositResult {
            log,
            new_balance
        }
    }

    pub fn set_notifer(&mut self, host: String, address2notify: String) {
        let notifer = Notifier {
            host,
            address2notify
        };

        self.notifer = Some(notifer);
    }

    pub fn update_balance(&mut self, balance: Balance) {
        self.balance = balance;
    }

    pub async fn notify(&self, notifcation: notify::Notification) -> Result<(), String>{
        if let Some(notifier) = self.notifer.borrow() {
            notifier.notify(notifcation).await
        } else {
            Err("notifer not set".into())
        }
    }
}

// used in manage canister 