use std::{cell::RefCell, collections::HashMap, borrow::BorrowMut, time::Duration};

use ic_cdk::caller;
use ic_cdk_macros::{init, query, update};
use candid::{Principal, Nat, candid_method};
use ePay_backend::{merchant::merchant::{Merchant}, management::state::{StateInfo, MerchantConfig}, merchant::{order::{Order, self}, self}, tokens::{TokenInfo, TokenType}, types::Account, interop::user::UserOp};

thread_local! {
    static STATE_INFO: RefCell<StateInfo> = RefCell::new(StateInfo::default());
    static MERCHNANT: RefCell<Merchant> = RefCell::new(Merchant::default());
}

#[init]
#[candid_method(init)]
fn init(owner: Principal, user_can: Principal, id: u64, conf: MerchantConfig) {
    let caller = ic_cdk::api::caller();
    STATE_INFO.with(|info| {
        let mut info = info.borrow_mut();
        info.add_manager(caller);

        info.user_canister = Some(user_can);
    });

    MERCHNANT.with(|merchant| {
        let mut merchant = merchant.borrow_mut();
        merchant.owner = owner;
        merchant.conf = conf.clone();
        merchant.id = id;
    });

    ic_cdk_timers::set_timer_interval(Duration::from_secs(conf.order_check_duration.clone()), || {
        MERCHNANT.with(|merchant| {
            let mut merchant = merchant.borrow_mut();
            merchant.check_orders_and_update();
        })
    });
}

#[query(guard = "is_merchant")]
#[candid_method(query)]
fn get_merchant_info() -> Merchant {
    MERCHNANT.with(|merchant| {
        let merchant = merchant.borrow();
        merchant.get_merchant_masked_off_orders()
    })
}

#[update(guard = "is_merchant")]
#[candid_method(update)]
fn publish_order(token_list: Vec<Principal>, token_standards: Vec<String>, token_amount: Vec<Nat>, payload: Option<Vec<u8>>, payload_spec: Option<String>, payer: Account) -> Result<u64, String>{
    let token_allowed = MERCHNANT.with(|merchant| {
        let merchant = merchant.borrow();
        merchant.conf.token_allowed.clone()
    });

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
        if !token_allowed.contains(&token_info) {
            return Err(format!("unsupported token used: {:?}", token_info).into());
        }

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

#[update(guard = "is_normal")]
#[candid_method(update)]
async fn pay_order(order_id: u64) -> Result<bool, String> {
    let caller = ic_cdk::caller();

    let (order, merchant_id) = MERCHNANT.with(|merchant| {
        let merchant = merchant.borrow_mut();
        match merchant.get_order(order_id) {
            Some(o) => (Some((*o).clone()), Some(merchant.id)),
            None => (None, None)
        } 
    });

    let user_can_principal = STATE_INFO.with(|info| {
        let info = info.borrow();
        info.user_canister.clone()
    });

    if let (Some(o), Some(m)) = (order, merchant_id) {
        if o.paid {
            Err("order paid".into())
        } else {
            match o.pay().await {
                Ok(paid) => {
                    MERCHNANT.with(|merhcant| {
                        let mut merchant = merhcant.borrow_mut();
                        match merchant.get_order_mut(order_id) {
                            Some(o) => o.mark_as_paid(), 
                            None => {}
                        };
                    });

                    if let Some(u) = user_can_principal {
                        let user_op = UserOp { principal: u };
                        user_op.attach_order2user(caller, m, o.id).await;
                    }

                    Ok(paid)
                },
                Err(e) => Err(e)
            }
        }
    } else {
        Err(format!("no such order: {}", order_id).into())
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
fn get_on_hold_orders() -> Vec<u64> {
    MERCHNANT.with(|merchant| {
        let merchant = merchant.borrow();
        merchant.orders_on_hold.clone()
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

#[update(guard = "is_authorized")]
#[candid_method(update)]
async fn refund_order(order_id: u64) -> Result<bool, String> {
    let order = MERCHNANT.with(|merchant| {
        let merchant = merchant.borrow_mut();
        match merchant.get_order(order_id) {
            Some(o) => Some((*o).clone()),
            None => None
        } 
    });

    match order {
        Some(o) => {
            if !o.paid {
                Err(format!("order unpaid: {}", o.id).into())
            } else {
                match o.refund().await {
                     Ok(paid) => {
                        if paid {
                            MERCHNANT.with(|merhcant| {
                                let mut merchant = merhcant.borrow_mut();
                                match merchant.get_order_mut(order_id) {
                                    Some(o) => o.mark_as_refunded(), 
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

#[update(guard = "is_authorized")]
#[candid_method(update)]
fn update_info(paylaod_spec: String, payload: Vec<u8>) {
    MERCHNANT.with(|merchant| {
        let mut merchant = merchant.borrow_mut();
        merchant.info = Some(payload);
        merchant.info_spec = Some(paylaod_spec)
    });
}

#[update(guard = "is_manager")]
#[candid_method(update)]
fn set_block(block: bool) -> bool {
    MERCHNANT.with(|merchant| {
        let mut merchant = merchant.borrow_mut();
        merchant.blocked = block;
    });
    true
}

#[update(guard = "is_manager")]
#[candid_method(update)]
fn set_verify(verified: bool) -> bool {
    MERCHNANT.with(|merchant| {
        let mut merchant = merchant.borrow_mut();
        merchant.verified = verified;
    });
    true
}

#[update(guard = "is_manager")]
#[candid_method(update)]
fn update_config(conf: MerchantConfig) {
    MERCHNANT.with(|merchant| {
        let mut merchant = merchant.borrow_mut();
        merchant.conf = conf;
    });
}

#[update(guard = "is_manager")]
#[candid_method(update)]
fn get_config() -> MerchantConfig {
    MERCHNANT.with(|merchant| {
        let merchant = merchant.borrow();
        merchant.conf.clone()
    })
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
        if merchant.owner != user {
            Err("no owner!".into())
        } else {
            Ok(())
        }
    })
}

fn is_authorized() -> Result<(), String> {
    let user = ic_cdk::api::caller();
    let mut authorized = false;
    STATE_INFO.with(|info| {
        let info = info.borrow();
        authorized |= info.is_manager(user);
    });

    MERCHNANT.with(|merchant| {
        let merchant = merchant.borrow();
        authorized |= merchant.owner == user;
    });

    if authorized {
        Ok(())
    } else {
        Err("not merchant or manager".into())
    }
}

fn is_normal() -> Result<(), String> {
    MERCHNANT.with(|merchant| {
        let merchant = merchant.borrow();
        if !merchant.blocked {
            Ok(())
        } else {
            Err("blocked by manager".into())
        }
    })
}