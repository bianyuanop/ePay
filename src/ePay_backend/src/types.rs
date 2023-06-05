use candid::{CandidType, Deserialize, Principal, types::principal};
use ic_ledger_types::{Subaccount};

#[derive(CandidType, Deserialize, Clone)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Subaccount>
}

impl From<Principal> for Account {
    fn from(value: Principal) -> Self {
        Account { owner: value, subaccount: None }
    }
}

#[cfg(test)]
mod test {
    use candid::Principal;
    use ic_ledger_types::Subaccount;

    fn principal2subaccount() {
        let principal = Principal::anonymous();
        let subaccount = Subaccount::from(principal);
        
        assert_eq!(1, 1);
    }
}