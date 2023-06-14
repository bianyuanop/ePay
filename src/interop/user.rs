use candid::Principal;

pub struct UserOp {
    pub principal: Principal,
}

impl UserOp {
    pub async fn attach_merchant2user(&self, user: Principal, merchant_id: u64) -> Result<(), String> {
        let call_res: Result<(Result<(), String>, ), _> = ic_cdk::call(self.principal, "attach_merchant2user", (user, merchant_id)).await;

        call_res.unwrap().0
    }

    pub async fn attach_order2user(&self, user: Principal, merchant_id: u64, order_id: u64) -> () {
        let call_res: Result<((), ), _> = ic_cdk::call(self.principal, "add_order", (user, order_id, merchant_id)).await;

        call_res.unwrap().0
    }
}
