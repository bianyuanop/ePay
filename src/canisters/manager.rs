use std::{cell::RefCell, collections::{HashSet, HashMap, BTreeMap}};
use ePay_backend::{management::{state::{StateInfo, MerchantDB, MerchantConfig}, wasm_store::WasmStorage, report::MerchantManageReport}, interop::{merchant::MerchantOp, user::UserOp}, utils::create_and_install_canister, tokens::{TokenInfo, TokenType}};

use ic_cdk_macros::{init, query, update, pre_upgrade, post_upgrade};
use candid::{candid_method, Principal};
use ic_cdk::{api::management_canister::main::{
    CreateCanisterArgument,
    CanisterSettings,
    InstallCodeArgument,
    create_canister,
    install_code,
    CanisterInstallMode,
    WasmModule,
}, storage};


thread_local! {
    static STATE_INFO: RefCell<StateInfo> = RefCell::new(StateInfo::default());
    static MERCHANT_DB: RefCell<MerchantDB> = RefCell::new(MerchantDB::default());
    static WASM_STORE: RefCell<WasmStorage> = RefCell::new(WasmStorage::default());
}

#[init]
#[candid_method(init)]
async fn init() {
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
            token_allowed: HashSet::new()
        };
    });
}

#[update(guard = "is_owner")]
#[candid_method(update)]
fn upload_wasm(name: String, wasm: Vec<u8>) -> Result<(), String> {
    WASM_STORE.with(|store| {
        let mut store = store.borrow_mut();
        if name.to_lowercase() == "merchant" {
            store.merchant_wasm = Some(wasm);
            Ok(()) 
        } else if name.to_lowercase() == "user" {
            store.user_wasm = Some(wasm);
            Ok(()) 
        } else {
            Err("no such wasm file needs to be uploaded".into())
        }
    })
}

#[update(guard = "is_owner")]
#[candid_method(update)]
async fn install_user_canister() -> Result<Principal, String> {
    let mut controllers = vec![ic_cdk::id()];
    STATE_INFO.with(|info| {
        let info = info.borrow();
        controllers.push(info.owner)
    });

    let canister_creation_arg = CreateCanisterArgument { 
        settings:Some(CanisterSettings {
            controllers: Some(controllers),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None
        })
    };

    if let Some(wasm_module) = WASM_STORE.with(|store| {
        let store: std::cell::Ref<WasmStorage> = store.borrow();
        store.user_wasm.clone()
    }) {
        let init_arg = candid::encode_args(()).unwrap();

        let install_res = create_and_install_canister(canister_creation_arg, init_arg, wasm_module).await;

        match install_res {
            Ok(canister_id) => {
                STATE_INFO.with(|info| {
                    let mut info = info.borrow_mut();
                    info.user_canister = Some(canister_id);
                });

                Ok(canister_id)
            },
            Err(e) => Err(e)
        }
    } else {
        Err("user wasm file not uploaded".into())
    }
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

#[update(guard = "is_authorized")]
#[candid_method(update)]
async fn set_merchant_block(merchant_id: u64, block: bool) -> Result<bool, String> {
    let merchant_principal = MERCHANT_DB.with(|db| {
        let db = db.borrow();
        db.get_merchant_principal(merchant_id)
    });

    match merchant_principal {
        Some(p) => {
            let merchant_op = MerchantOp {principal: p};
            Ok(merchant_op.set_merchant_block(block).await)
        },
        None => Err(format!("no such merchant: {}", merchant_id).into())
    }
}

#[update(guard = "is_authorized")]
#[candid_method(update)]
async fn set_merchant_verified(merchant_id: u64, verified: bool) -> Result<bool, String> {
    let merchant_principal = MERCHANT_DB.with(|db| {
        let db = db.borrow();
        db.get_merchant_principal(merchant_id)
    });

    match merchant_principal {
        Some(p) => {
            let merchant_op = MerchantOp {principal: p};
            Ok(merchant_op.set_merchant_verify(verified).await)
        },
        None => Err(format!("no such merchant: {}", merchant_id).into())
    }
}

#[update(guard = "is_authorized")]
#[candid_method(update)]
async fn update_allowed_token_list(token_list: Vec<Principal>, token_standards: Vec<String>) -> Result<(), String> {
    let n = token_list.len();
    if token_standards.len() < n {
        return Err("length of token standards not match with token list".into());
    }

    let mut list2update = HashSet::new();

    for i in 0..n {
        let standard = TokenType::from(token_standards[i].clone());
        let token_info = TokenInfo::generate_token_info(token_list[i], standard);

        list2update.insert(token_info);
    }

    STATE_INFO.with(|info| {
        let mut info = info.borrow_mut();
        info.merchant_conf.token_allowed = list2update;
    });

    Ok(())
}

#[update(guard = "is_authorized")]
#[candid_method(update)]
async fn get_merchant_conf(merchant_id: u64) -> Result<MerchantConfig, String> {
    let merchant_principal = MERCHANT_DB.with(|db| {
        let db = db.borrow();
        db.get_merchant_principal(merchant_id)
    });

    match merchant_principal {
        Some(p) => {
            let merchant_op = MerchantOp {principal: p};
            let config = merchant_op.get_conf().await;
            Ok(config)
        },
        None => Err("no such merchant".into())
    }
}

#[update(guard = "is_authorized")]
#[candid_method(update)]
async fn update_merchant_conf(merchant_id: u64, conf: MerchantConfig) -> Result<(), String> {
    let merchant_principal = MERCHANT_DB.with(|db| {
        let db = db.borrow();
        db.get_merchant_principal(merchant_id)
    });

    match merchant_principal {
        Some(p) => {
            let merchant_op = MerchantOp {principal: p};
            merchant_op.update_config(conf).await
        },
        None => Err("no such merchant".into())
    }
}

#[update(guard = "is_authorized")]
#[candid_method(update)]
async fn update_all_merchant_allowed_tokens() -> MerchantManageReport {
    let merchants = MERCHANT_DB.with(|db| {
        let db = db.borrow();
        db.merchants.clone()
    });

    let token_allowed = STATE_INFO.with(|info| {
        let info = info.borrow();
        info.merchant_conf.token_allowed.clone()
    });

    let mut mmp = MerchantManageReport::default();
    for (id, principal) in merchants.iter() {
        let merchant_op = MerchantOp {principal: *principal};
        let mut config = merchant_op.get_conf().await;
        config.token_allowed  = token_allowed.clone();
        let update_res = merchant_op.update_config(config).await;
        
        if update_res.is_err() {
            mmp.insert_record(*id, update_res.err().unwrap());
        }
    }

    mmp
}


#[update]
#[candid_method(update)]
async fn create_merchant(owner: Principal) -> Result<u64, String> {
    if let Some(wasm_module) = WASM_STORE.with(|store| {
        let store = store.borrow();
        store.merchant_wasm.clone()
    }) {
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
        let user_canister_principal = STATE_INFO.with(|info| {
            let info = info.borrow();
            info.user_canister
        });

        if user_canister_principal.is_none() {
            return Err("user canister not installed yet".into());
        }

        let merchant_id = MERCHANT_DB.with(|db| {
            let mut db = db.borrow_mut();
            // since merchant id allocation relies on atomiciy and only operations between await func calls are atomic
            // see https://forum.dfinity.org/t/how-to-ensure-atomicity-when-calling-a-canister-function/7602
            // here we insert an anonymous principal first then update it after canister creation
            db.add_merchant(Principal::anonymous())
        });

        // see `merchant.did`
        // TODO: destroy on failure
        let init_arg = candid::encode_args((owner, user_canister_principal.unwrap(), merchant_id, merchant_conf,)).unwrap();
        let install_res = create_and_install_canister(canister_creation_arg, init_arg, wasm_module).await;

        match install_res {
            Ok(canister_id) => {

                MERCHANT_DB.with(|db| {
                    let mut db = db.borrow_mut();
                    db.update_merchant(merchant_id, canister_id);
                });

                let user_op = STATE_INFO.with(|info| {
                    let info = info.borrow();
                    if let Some(user_can) = info.user_canister {
                        let op = UserOp { principal: user_can};
                        Some(op)
                    } else {
                        None
                    }
                });

                if let Some(op) = user_op {
                    match op.attach_merchant2user(owner, merchant_id).await {
                        Ok(_) => {
                            Ok(merchant_id)
                        },
                        Err(e) => {
                            Err(e)
                        }
                    }
                } else {
                    Err("user canister not installed".into())
                }
            }, 
            Err(e) => Err(e)
        }
    } else {
        Err("merchant wasm not uploaded".into())
    }
}

#[query]
#[candid_method(query)]
fn get_user_canister() -> Option<Principal> {
    STATE_INFO.with(|info| {
        let info = info.borrow();
        info.user_canister
    })
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