use std::{cell::RefCell, collections::HashMap};

use ic_cdk_macros::{init, query, update};
use ic_cdk::export::candid::candid_method;
use candid::{CandidType, Deserialize, Nat, Principal, types::principal};

use ePay_backend::{user::user::{User, UserDB}, management::state::StateInfo};


thread_local! {
    static USERDB: RefCell<UserDB> = RefCell::new(UserDB::new());
    static STATE_INFO: RefCell<StateInfo> = RefCell::new(StateInfo::default());
}


// #[init]
// #[candid_method(init)]
fn init() {
    let caller = ic_cdk::api::caller();
    STATE_INFO.with(|info| {
        let mut info = info.borrow_mut();
        info.add_manager(caller);
    });
}

fn register(user: Principal) {
    USERDB.with(|db| {
        let mut db = db.borrow_mut();
        db.generate_user_and_insert(user);
    })
}

fn add_order(user_id: Principal, order_id: u64, merchant_id: u64) -> Result<bool, String> {
    USERDB.with(|db| {
        let mut db = db.borrow_mut();
        let user = db.get_user_mut(user_id);
        match user {
            Some(o) => {
                o.add_order(merchant_id, order_id);
                Ok(true)
            },
            None => Err(format!("no such user: {}", user_id).into())
        }
    })
}

fn get_user(user: Principal) -> Option<User> {
    let caller = ic_cdk::caller();
    if user == caller {
        USERDB.with(|db| {
            let db = db.borrow();
            match db.get_user(&user) {
                Some(u) => {
                    Some((*u).clone())
                },
                None => None
            }
        })
    } else {
        match is_authorized() {
            Ok(_) => {
                USERDB.with(|db| {
                    let db = db.borrow();
                    match db.get_user(&user) {
                        Some(u) => {
                            Some((*u).clone())
                        },
                        None => None
                    }
                })
            },
            Err(_) => None
        }
    }
}

fn request_merchant(user: Principal) -> Result<bool, String> {
    Ok(true)
}


fn main() {
    candid::export_service!();
    std::println!("{}", __export_service());
}

// helper functions below
fn is_authorized() -> Result<(), String> {
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