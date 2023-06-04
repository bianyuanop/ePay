use std::cell::RefCell;
use ePay_backend::{merchant::merchant::MerchantDB};

thread_local! {
    static MERCHANT_DB: RefCell<MerchantDB> = RefCell::new(MerchantDB::default());
}

fn main() {
    candid::export_service!();
    std::println!("{}", __export_service());
}