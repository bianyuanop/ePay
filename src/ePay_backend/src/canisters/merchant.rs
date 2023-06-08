use std::{cell::RefCell, collections::HashMap, borrow::BorrowMut, time::Duration};

use ic_cdk::caller;
use ic_cdk_macros::{init, query, update};
use candid::{Principal, Nat, candid_method};
use ePay_backend::{merchant::merchant::{Merchant}, management::state::{StateInfo, MerchantConfig}, merchant::{order::{Order, self}, self}, tokens::{TokenInfo, TokenType}, types::Account};

thread_local! {
    static STATE_INFO: RefCell<StateInfo> = RefCell::new(StateInfo::default());
    static MERCHNANT: RefCell<Merchant> = RefCell::new(Merchant::default());
}

// #[init]
// #[candid_method(init)]
fn init(owner: Principal, conf: MerchantConfig) {
    let caller = ic_cdk::api::caller();
    STATE_INFO.with(|info| {
        let mut info = info.borrow_mut();
        info.add_manager(caller);
    });

    MERCHNANT.with(|merchant| {
        let mut merchant = merchant.borrow_mut();
        merchant.owner = owner;
        merchant.conf = conf.clone();
    });

    ic_cdk_timers::set_timer_interval(Duration::from_secs(conf.order_check_duration.clone()), || {
        MERCHNANT.with(|merchant| {
            let mut merchant = merchant.borrow_mut();
            merchant.check_orders_and_update();
        })
    });
}

#[update(guard = "is_merchant")]
#[candid_method(update)]
fn publish_order(token_list: Vec<Principal>, token_standards: Vec<String>, token_amount: Vec<Nat>, payload: Option<Vec<u8>>, payload_spec: Option<String>, payer: Account) -> Result<u64, String>{
    let n = token_list.len();
    if token_standards.len() < n || token_amount.len() < n {
        return Err("token standards or token amount array incorrect length".into());
    }

    let mut tokens_needed : HashMap<TokenInfo, Nat> = HashMap::new(); 
    for i in 0..n {
        let standard: Option<TokenType> = match token_standards[i].as_str() {
            "ICRC-1" => Some(TokenType::ICRC1),
            "DIP20" => Some(TokenType::DIP20),
            _ => Some(TokenType::OTHER)
        };

        if standard == None {
            return Err(format!("unsupported token standard: {}", token_standards[i]).into());
        }

        let token_info = TokenInfo::generate_token_info(token_list[i], standard.unwrap());

        tokens_needed.insert(token_info, token_amount[i].clone());
    }


    MERCHNANT.with(|merchant| {
        let mut merchant = merchant.borrow_mut();
        let order = Order::generate_order(merchant.order_ptr, ic_cdk::api::time(), tokens_needed, payload, payload_spec, ic_cdk::api::id(), payer);

        let res_id = merchant.add_order(order);
        Ok(res_id)
    })
}

#[query]
#[candid_method(query)]
fn view_order(order_id: u64) -> Result<Order, String> {
    MERCHNANT.with(|merchant| {
        let merchant = merchant.borrow();
        let order = merchant.get_order(order_id).clone();
        match order {
            Some(o) => Ok(o.clone()),
            None => Err(format!("no such order: {}", order_id).into())
        }
    })
}

#[update]
#[candid_method(update)]
async fn pay_order(order_id: u64) -> Result<bool, String> {
    let caller = ic_cdk::caller();
    let order = MERCHNANT.with(|merchant| {
        let merchant = merchant.borrow_mut();
        match merchant.get_order(order_id) {
            Some(o) => Some((*o).clone()),
            None => None
        } 
    });

    match order {
        Some(o) => {
            if o.paid {
                Ok(true)
            } else {
                match o.pay().await {
                     Ok(paid) => {
                        if paid {
                            MERCHNANT.with(|merhcant| {
                                let mut merchant = merhcant.borrow_mut();
                                match merchant.get_order_mut(order_id) {
                                    Some(o) => o.mark_as_paid(), 
                                    None => {}
                                };
                            });
                        }
                        Ok(paid)
                    },
                    Err(e) => Err(e)
                }
            }
        }, 
        None => Err(format!("no such order: {}", order_id).into())
    }
}

#[query]
#[candid_method(query)]
async fn order_paid(order_id: u64) -> Result<bool, String> {
    MERCHNANT.with(|merchant| {
        let merchant = merchant.borrow();
        match merchant.get_order(order_id) {
            Some(o) => Ok(o.paid),
            None => Err(format!("no such order: {}", order_id).into())
        }
    })
}

#[update]
#[candid_method(update)]
fn comment_order(order_id: u64, payload: Vec<u8>, payload_spec: String) -> Result<bool, String> {
    MERCHNANT.with(|merchant| {
        let mut merchant = merchant.borrow_mut();
        let order = merchant.get_order_mut(order_id);
        match order {
            Some(o) => {
                o.insert_comment(ic_cdk::caller(), payload, payload_spec);
                Ok(true)
            },
            None => Err(format!("no such order: {}", order_id).into())
        }
    })
}

#[query]
#[candid_method(query)]
fn owner() -> Principal {
    MERCHNANT.with(|merchant| {
        let merchant = merchant.borrow();
        merchant.owner
    })
}

// this functionality seems can't be implemented with ICRC-1 standards since it has no transaction log
// can implement with future ICRC standards
#[allow(unused)]
async fn refund_order(order_id: u64) -> Result<bool, String> {
    unimplemented!()
}


fn main() {
    candid::export_service!();
    std::println!("{}", __export_service());
}

// helper functions below
fn is_manager() -> Result<(), String> {
    let user = ic_cdk::api::caller();
    STATE_INFO.with(|info| {
        let info = info.borrow();
        if !info.is_manager(user) {
            Err("unauthorized!".into())
        } else {
            Ok(())
        }
    })
}

fn is_merchant() -> Result<(), String> {
    let user = ic_cdk::api::caller();
    MERCHNANT.with(|merchant| {
        let merchant = merchant.borrow();
        if merchant.owner != caller() {
            Err("no owner!".into())
        } else {
            Ok(())
        }
    })
}