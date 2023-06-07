#!/usr/bin/ic-repl
load "prelude.sh";
load "dip20.sh";

import fake = "2vxsx-fae" as "../src/ePay_backend/src/canisters/merchant.did";
let wasm = file("../target/wasm32-unknown-unknown/release/merchant_opt.wasm");

identity token_minter "~/.config/dfx/identity/test/identity.pem"
identity bob;

identity alice;
let args = encode fake.__init_args(
  alice
);

let MER = install(wasm, args, null);

call MER.owner();

assert (_ : principal) == alice;

