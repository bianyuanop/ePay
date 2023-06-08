use candid::{Principal, Nat};

pub struct Merchant {
    principal: Principal,
}

impl Merchant {
    pub fn new(merchant: Principal) -> Self {
        Self {
            principal: merchant
        }
    }

    pub fn refund_order(&self, order_id: Nat) {
       unimplemented!() 
    }
}
