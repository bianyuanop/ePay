#!/usr/bin/ic-repl
load "../tests/prelude.sh";

let merchant_wasm = file("../target/wasm32-unknown-unknown/release/merchant_opt.wasm");
let user_wasm = file("../target/wasm32-unknown-unknown/release/user_opt.wasm");

identity test "~/.config/dfx/identity/test/identity.pem";

let MANAGER = service "bd3sg-teaaa-aaaaa-qaaba-cai";

call MANAGER.install_user_canister();
let USER = _[17_724];

call USER.register(test);

identity test;
call MANAGER.create_merchant(test);
