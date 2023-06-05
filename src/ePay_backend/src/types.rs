use candid::{CandidType, Deserialize, Principal, types::principal};
use ic_ledger_types::{Subaccount};

#[derive(CandidType, Deserialize, Clone)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Subaccount
}

impl From<Principal> for Account {
    fn from(value: Principal) -> Self {
        Account { owner: value, subaccount: Subaccount::from(value) }
    }
}