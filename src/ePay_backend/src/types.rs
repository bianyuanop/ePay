use candid::{CandidType, Deserialize, Principal};
use ic_ledger_types::{Subaccount};

#[derive(CandidType, Deserialize)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Subaccount
}