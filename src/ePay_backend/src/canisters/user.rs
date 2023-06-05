use std::{cell::RefCell, collections::HashMap};

use ic_cdk_macros::{init, query, update};
use ic_cdk::export::candid::candid_method;
use candid::{CandidType, Deserialize, Nat, Principal};

use ePay_backend::{user::user::User, management::state::StateInfo};


thread_local! {
    static USERS: RefCell<HashMap<Nat, User>> = RefCell::new(HashMap::new());
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

fn register() {

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