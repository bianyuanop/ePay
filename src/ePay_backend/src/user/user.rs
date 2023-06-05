use std::{collections::BTreeMap, vec};

use candid::{Deserialize, CandidType, Nat, Principal};

use crate::{tokens::dip20::Metadata};

use super::balance::Balance;

#[derive(CandidType, Deserialize)]
pub struct User {
    id: Nat,
    username: Option<String>,
    principal: Principal,
    blocked: bool,
    balance: Balance,

    orders: Vec<u64>,
}

impl From<Principal> for User {
    fn from(principal: Principal) -> Self {
        Self {
            id: Nat::default(),
            username: None,
            principal,
            blocked: false,
            balance: Balance::default(),
            orders: vec![]
        }
    }
}


#[derive(CandidType, Deserialize, Default)]
pub struct UserDB {
    p: u64,
    users: BTreeMap<u64, User>,
}

impl UserDB {
    pub fn new() -> Self {
        Self { p: 0, users: BTreeMap::default() }
    }
    
    pub fn add_user(&mut self, user: User) {
        self.users.insert(self.p, user);
        self.p = self.p + 1;
    }

    pub fn block_user(&mut self, id: u64) -> bool {
        if self.users.contains_key(&id) {
            let mut user = self.users.get_mut(&id).unwrap();
            user.blocked = true;
            true
        } else {
            false
        }
    }
}