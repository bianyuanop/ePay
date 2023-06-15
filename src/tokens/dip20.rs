use std::ops::Sub;

// The implementation is copied and altered from https://github.com/dfinity/examples/blob/38c254513c816055d8b653b586804ccb6c6dfcb7/rust/defi/src/defi_dapp/dip20.rs
use candid::{CandidType, Deserialize, Nat, Principal};

pub struct DIP20 {
    principal: Principal,
}

#[derive(CandidType, Debug, PartialEq, Deserialize)]
pub enum TxError {
    InsufficientAllowance,
    InsufficientBalance,
    ErrorOperationStyle,
    Unauthorized,
    LedgerTrap,
    ErrorTo,
    Other(String),
    BlockUsed,
    AmountTooSmall,
}
pub type TxReceipt = Result<Nat, TxError>;

#[allow(non_snake_case)]
#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct Metadata {
    pub logo: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub totalSupply: Nat,
    pub owner: Principal,
    pub fee: Nat,
}

impl DIP20 {
    pub fn new(principal: Principal) -> Self {
        DIP20 { principal }
    }

    pub async fn balance_of(&self, account: Principal) -> Nat {
        let call_result: Result<(Nat,), _> = 
            ic_cdk::api::call::call(self.principal, "balanceOf", (account,)).await;

        call_result.unwrap().0
    }

    pub async fn transfer(&self, target: Principal, amount: Nat) -> TxReceipt {
        let call_result: Result<(TxReceipt,), _> =
            ic_cdk::api::call::call(self.principal, "transfer", (target, amount)).await;

        call_result.unwrap().0
    }

    pub async fn transfer_from(
        &self,
        source: Principal,
        target: Principal,
        amount: Nat,
    ) -> TxReceipt {
        let call_result: Result<(TxReceipt,), _> =
            ic_cdk::api::call::call(self.principal, "transferFrom", (source, target, amount)).await;

        call_result.unwrap().0
    }

    pub async fn allowance(&self, owner: Principal, spender: Principal) -> Nat {
        let call_result: Result<(Nat,), _> =
            ic_cdk::api::call::call(self.principal, "allowance", (owner, spender)).await;

        call_result.unwrap().0
    }

    pub async fn get_metadata(&self) -> Metadata {
        let call_result: Result<(Metadata,), _> =
            ic_cdk::api::call::call(self.principal, "getMetadata", ()).await;

        call_result.unwrap().0
    }

    pub fn calculate_transferable(amount: Nat, fee: Nat) -> Nat {
        return amount.sub(fee).sub(Nat::from(1));
    }
}