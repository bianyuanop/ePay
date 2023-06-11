use candid::Principal;

use crate::management::state::MerchantConfig;

pub struct MerchantOp {
    pub principal: Principal,
}

impl MerchantOp {
    pub async fn refund_order(&self, order_id: u64) -> Result<bool, String> {
        let call_res: Result<(Result<bool, String>,), _> = ic_cdk::api::call::call(self.principal, "refund_order", (order_id,)).await;

        call_res.unwrap().0
    }

    pub async fn set_merchant_block(&self, block: bool) -> bool {
        let call_res: Result<(bool,), _> = ic_cdk::api::call::call(self.principal, "set_block", (block,)).await;

        call_res.unwrap().0
    }

    pub async fn set_merchant_verify(&self, verified: bool) -> bool {
        let call_res: Result<(bool,), _> = ic_cdk::api::call::call(self.principal, "set_verify", (verified,)).await;

        call_res.unwrap().0
    }

    pub async fn get_conf(&self) -> MerchantConfig {
        let call_res: Result<(MerchantConfig,), _> = ic_cdk::api::call::call(self.principal, "get_config", ()).await;

        call_res.unwrap().0
    }

    pub async fn update_config(&self, config: MerchantConfig) -> Result<(), String> {
        let call_res: Result<(), _> = ic_cdk::api::call::call(self.principal, "update_config", (config,)).await;

        match call_res {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{:?}", e).into())
        }
    }
}
