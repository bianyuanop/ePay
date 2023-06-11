use std::{collections::{BTreeMap, HashSet}, vec};

use candid::{Deserialize, CandidType, Nat, Principal};

#[derive(CandidType, Deserialize, Clone)]
pub struct OrderBrief {
    pub order_id: u64,
    pub merchant_id: u64,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct User {
    pub id: Nat,
    pub username: Option<String>,
    pub principal: Principal,
    pub blocked: bool,

    // owned merchants IDs
    pub merchants: HashSet<u64>,

    orders: Vec<OrderBrief>,
}

impl From<Principal> for User {
    fn from(principal: Principal) -> Self {
        Self {
            id: Nat::default(),
            username: None,
            principal,
            blocked: false,
            orders: vec![],
            merchants: HashSet::new()
        }
    }
}

impl User {
    pub fn add_order(&mut self, merchant_id: u64, order_id: u64) {
        self.orders.push(OrderBrief {
            order_id,
            merchant_id
        })
    } 
}


#[derive(CandidType, Deserialize, Default)]
pub struct UserDB {
    p: u64,
    users: BTreeMap<Principal, User>,
}

impl UserDB {
    pub fn generate_user_and_insert(&mut self, principal: Principal) {
        let user = User {
            id: Nat::from(self.p.clone()),
            username: None,
            principal,
            blocked: false,
            orders: vec![],
            merchants: HashSet::new()
        };
        self.users.insert(principal, user);
    }

    pub fn new() -> Self {
        Self { p: 0, users: BTreeMap::default() }
    }
    
    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.principal, user);
        self.p = self.p + 1;
    }

    pub fn get_user(&self, user: &Principal) -> Option<&User> {
        self.users.get(user) 
    }

    pub fn block_user(&mut self, id: &Principal) -> bool {
        if self.users.contains_key(id) {
            let mut user = self.users.get_mut(&id).unwrap();
            user.blocked = true;
            true
        } else {
            false
        }
    }

    pub fn get_user_mut(&mut self, user_id: Principal) -> Option<&mut User> {
        self.users.get_mut(&user_id)
    }

    pub fn has_user(&self, user_id: Principal) -> bool {
        self.users.contains_key(&user_id)
    }
}