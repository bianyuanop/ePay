use std::cell::RefCell;
use ePay_backend::{management::{state::{StateInfo, MerchantDB, MerchantConfig}, merchant::{self, Merchant}}, consts::MERCHANT_WASM};

use ic_cdk_macros::{init, query, update};
use candid::{candid_method, Principal};
use ic_cdk::api::management_canister::main::{
    CreateCanisterArgument,
    CanisterSettings,
    InstallCodeArgument,
    create_canister,
    install_code,
    CanisterInstallMode,
    WasmModule,
};


thread_local! {
    static STATE_INFO: RefCell<StateInfo> = RefCell::new(StateInfo::default());
    static MERCHANT_DB: RefCell<MerchantDB> = RefCell::new(MerchantDB::default());
}

#[init]
#[candid_method(init)]
fn init() {
    let caller = ic_cdk::api::caller();
    STATE_INFO.with(|info| {
        let mut info = info.borrow_mut();
        info.add_manager(caller);
        info.set_owner(caller);
        info.merchant_conf = MerchantConfig {
            order_check_duration: 60*60,
            order_on_hold_duration: 60*60*24*7,
            fee_rate: 0.001,
            fee_to: caller,
        };
    });
}

#[update(guard = "is_owner")]
#[candid_method(update)]
fn add_manager(manager: Principal) -> Result<bool, String> {
    STATE_INFO.with(|state| {
        let mut state = state.borrow_mut();
        Ok(state.add_manager(manager))
    })
}

#[update(guard = "is_owner")]
#[candid_method(update)]
fn remove_manager(manager: Principal) -> Result<bool, String> {
    STATE_INFO.with(|state| {
        let mut state = state.borrow_mut();
        Ok(state.remove_manager(manager))
    })
}

#[query]
#[candid_method(query)]
fn get_merchant_by_id(id: u64) -> Option<Principal> {
    MERCHANT_DB.with(|db| {
        let db = db.borrow();
        db.get_merchant_principal(id)
    })
}

// this method should be called from `user` canister, which can associate merchant principal with the owner
#[update]
#[candid_method(update)]
async fn create_merchant(owner: Principal) -> Result<u64, String> {
    let mut manager = Principal::anonymous();
    let mut merchant_conf = MerchantConfig::default();
    STATE_INFO.with(|info| {
        let info = info.borrow();
        manager = info.owner.clone();
        merchant_conf = info.merchant_conf.clone();
    });
    // canister creation
    let canister_creation_arg = CreateCanisterArgument { 
        settings:Some(CanisterSettings {
            controllers: Some(vec![manager, ic_cdk::id()]),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None
        })
    };
    let canister_id = create_canister(canister_creation_arg).await.ok().unwrap().0.canister_id;

    // canister install
    let wasm_module = WasmModule::from(MERCHANT_WASM);
    // see `merchant.did`
    let init_arg = candid::encode_args((owner, merchant_conf)).unwrap();
    let install_arg = InstallCodeArgument {
        mode: CanisterInstallMode::Install,   
        canister_id,
        wasm_module,
        arg: init_arg
    };
    let install_res = install_code(install_arg).await;

    match install_res {
        Ok(_) => {
            let mut merchant_id: u64 = 0;
            MERCHANT_DB.with(|db| {
                let mut db = db.borrow_mut();
                merchant_id = db.add_merchant(canister_id);
            });

            Ok(merchant_id)
        },
        Err(e) => {
            Err(format!("{:?}", e).into())
        }
    }
}


fn main() {
    candid::export_service!();
    std::println!("{}", __export_service());
}

// helper functions below
fn is_authorized() -> Result<(), String> {
    let user = ic_cdk::api::caller();
    STATE_INFO.with(|info| {
        let info = info.borrow();
        if !info.is_manager(user) {
            Err("unauthorized!".into())
        } else {
            Ok(())
        }
    })
}

fn is_owner() -> Result<(), String> {
    let user = ic_cdk::api::caller();
    STATE_INFO.with(|info| {
        let info = info.borrow();
        if !info.is_owner(user) {
            Err("unauthorized!".into())
        } else {
            Ok(())
        }
    })
}