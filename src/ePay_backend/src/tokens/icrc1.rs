use candid::{CandidType, Deserialize, Principal, Nat};
use ic_ledger_types::{Timestamp, Subaccount};
use ic_cdk;

pub struct ICRC1 {
    principal: Principal
}

#[allow(non_snake_case)]
#[derive(CandidType, Debug, Deserialize)]
pub enum TxError {
    BadFee { expected_fee : Nat },
    BadBurn { min_burn_amount : Nat },
    Insufficie { balance : Nat },
    TooOld,
    CreatedInFuture{ ledger_time : Timestamp },
    Duplicate { duplicate_of : Nat },
    TemporarilyUnavailable,
    GenericError { error_code : Nat, message : String },
}

#[derive(CandidType, Deserialize)]
pub struct Account {
    owner: Principal,
    subaccount: Subaccount
}

#[derive(CandidType, Deserialize)]
pub struct TransferArgs {
    from_subaccount: Subaccount,
    to: Account,
    amount: Nat,
    fee: Option<Nat>,
    memo: Option<Vec<u8>>,
    created_at_time: Option<u64>
}

#[derive(CandidType, Deserialize)]
pub enum TransferResult {
    Ok(Nat),
    Err(TxError)
}

impl ICRC1 {
    pub async fn name(&self) -> String {
        let call_result: Result<(String, ), _> = 
            ic_cdk::api::call::call(self.principal, "icrc1_name", ()).await;
        
        call_result.unwrap().0
    }
    
    pub async fn transfer(&self, transfer_args: TransferArgs) -> TransferResult {
        let call_result: Result<(TransferResult,), _> = 
            ic_cdk::api::call::call(self.principal, "icrc1_transfer", (transfer_args,)).await;

        call_result.unwrap().0
    }

    // pub async fn allowance(&self, )
}