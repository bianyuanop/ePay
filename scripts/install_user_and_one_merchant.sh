#!/usr/bin/ic-repl
load "info.sh";

let merchant_wasm = file("../target/wasm32-unknown-unknown/release/merchant_opt.wasm");
let user_wasm = file("../target/wasm32-unknown-unknown/release/user_opt.wasm");

call MANAGER.install_user_canister();
let USER = _[17_724];

call USER.register(test);

identity test;
call MANAGER.create_merchant(test);
