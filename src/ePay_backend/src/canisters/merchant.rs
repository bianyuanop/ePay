use std::{cell::RefCell, collections::HashMap};

use candid::{Principal, Nat};
use ePay_backend::{merchant::merchant::{MerchantDB, Merchant}, management::state::StateInfo, payment::order::Order, tokens::{TokenInfo, TokenType}};

thread_local! {
    static STATE_INFO: RefCell<StateInfo> = RefCell::new(StateInfo::default());
    static MERCHNANT: RefCell<Merchant> = RefCell::new(Merchant::new());
}

fn init() {
    let caller = ic_cdk::api::caller();
    STATE_INFO.with(|info| {
        let mut info = info.borrow_mut();
        info.add_manager(caller);
    });
}

fn publish_order(token_list: Vec<Principal>, token_standards: Vec<String>, token_amount: Vec<Nat>, payload: Option<Vec<u8>>, payload_spec: Option<String>) -> Result<u64, String>{
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

    let mut res_id = 0;

    MERCHNANT.with(|merchant| {
        let mut merchant = merchant.borrow_mut();
        let order = Order::generate_order(merchant.order_ptr, ic_cdk::api::time(), tokens_needed, payload, payload_spec);

        res_id = merchant.add_order(order)
    });
    
    Ok(res_id)
}

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