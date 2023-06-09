#!/usr/bin/ic-repl

let merchant_wasm = file("../target/wasm32-unknown-unknown/release/merchant_opt.wasm");
let user_wasm = file("../target/wasm32-unknown-unknown/release/user_opt.wasm");

let MANAGER = service "bd3sg-teaaa-aaaaa-qaaba-cai";

identity test "~/.config/dfx/identity/test/identity.pem";

call MANAGER.upload_wasm("merchant", merchant_wasm);
call MANAGER.upload_wasm("user", user_wasm);