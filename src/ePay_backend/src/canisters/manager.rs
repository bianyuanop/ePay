use std::cell::RefCell;
use ePay_backend::{management::state::{StateInfo, MerchantDB}};

use ic_cdk_macros::{init, query, update};
use candid::{candid_method, Principal};

thread_local! {
    static STATE_INFO: RefCell<StateInfo> = RefCell::new(StateInfo::default());
    static MERCHANT_DB: RefCell<MerchantDB> = RefCell::new(MerchantDB::default());
}

#[init]
#[candid_method(init)]
fn init() {
    let caller = ic_cdk::api::caller();
    STATE_INFO.with(|info| {
        let mut info = info.borrow_mut();
        info.add_manager(caller);
        info.set_owner(caller);
    });
}

#[update(guard = "is_owner")]
#[candid_method(update)]
fn add_manager(manager: Principal) -> Result<bool, String> {
    STATE_INFO.with(|state| {
        let mut state = state.borrow_mut();
        Ok(state.add_manager(manager))
    })
}

#[update(guard = "is_owner")]
#[candid_method(update)]
fn remove_manager(manager: Principal) -> Result<bool, String> {
    STATE_INFO.with(|state| {
        let mut state = state.borrow_mut();
        Ok(state.remove_manager(manager))
    })
}

fn create_merchant(owner: Principal) -> Result<bool, String> {
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

fn is_owner() -> Result<(), String> {
    let user = ic_cdk::api::caller();
    STATE_INFO.with(|info| {
        let info = info.borrow();
        if !info.is_owner(user) {
            Err("unauthorized!".into())
        } else {
            Ok(())
        }
    })
}