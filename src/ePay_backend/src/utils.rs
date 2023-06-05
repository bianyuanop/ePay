use candid::Principal;
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