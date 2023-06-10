use std::collections::HashMap;
use candid::{Deserialize, CandidType};

#[derive(Deserialize, CandidType)]
pub struct MerchantManageReport {
    report: HashMap<u64, String>,
    timestamp: u64
}

impl Default for MerchantManageReport {
    fn default() -> Self {
        Self {
            report: HashMap::new(),
            timestamp: ic_cdk::api::time() 
        }
    }
}

impl MerchantManageReport {
    pub fn insert_record(&mut self, id: u64, content: String) {
        self.report.insert(id, content);
    }
}