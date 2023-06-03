use candid::Principal;
use ic_ledger_types::Subaccount;

use crate::types::Account;

pub fn pad_principal_to_subaccount(principal: Principal) -> Account {
    Account {
        owner: principal,
        subaccount: Subaccount::from(principal)
    }
}