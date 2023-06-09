use candid::Principal;
use ic_cdk::api::management_canister::{main::{CreateCanisterArgument, InstallCodeArgument, create_canister, install_code, CanisterInstallMode}, provisional::CanisterId};
use ic_ledger_types::{Subaccount, AccountIdentifier};

use crate::types::Account;

pub fn order_id_to_subaccount(id: u64) -> Subaccount {
    let mut subaccount = [0u8;32];
    subaccount.split_at_mut(8).0.copy_from_slice(id.to_le_bytes().as_slice());
    Subaccount(subaccount)
}

pub fn subaccount_to_order_id(subaccount: Subaccount) -> u64 {
    let mut arr: [u8; 8] = [0; 8];
    for i in 0..8 {
        arr[i] = subaccount.0[i];
    }

    u64::from_le_bytes(arr)
}

pub async fn create_and_install_canister(create_canister_arg: CreateCanisterArgument, init_arg: Vec<u8>, wasm_module: Vec<u8>) -> Result<CanisterId, String> {
    let canister_id = create_canister(create_canister_arg).await.ok().unwrap().0.canister_id;

    let install_arg = InstallCodeArgument {
        mode: CanisterInstallMode::Install,   
        canister_id,
        wasm_module,
        arg: init_arg
    };
    let install_res = install_code(install_arg).await;

    match install_res {
        Ok(_) => {
            Ok(canister_id)
        },
        Err(e) => {
            Err(format!("{:?}", e).into())
        }
    }
}

mod tests {
    use crate::utils::subaccount_to_order_id;

    use super::order_id_to_subaccount;

    #[test]
    fn test_order_id_subaccount_conversion() {
        let id: u64 = 20;
        let subacc = order_id_to_subaccount(id);

        assert_eq!(id, subaccount_to_order_id(subacc));
    }
}