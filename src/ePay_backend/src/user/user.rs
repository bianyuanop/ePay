use candid::{Deserialize, CandidType, Nat, Principal};

use crate::payment::order::OrderIndex;

use super::balance::Balance;


#[derive(CandidType, Deserialize)]
pub enum UserType {
    Merchant, 
    Normal
}

#[derive(CandidType, Deserialize)]
pub struct UserMetaData {
    user_type: UserType,
}

#[derive(CandidType, Deserialize)]
pub struct User {
    username: String,
    principal: Principal,
    metadata: UserMetaData,
    blocked: bool,
    balance: Balance,

    orders: Vec<OrderIndex>,
}