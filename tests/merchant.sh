#!/usr/bin/ic-repl
load "prelude.sh";
load "dip20.sh";

import fake = "be2us-64aaa-aaaaa-qaabq-cai" as "../src/canisters/merchant.did";
let wasm = file("../target/wasm32-unknown-unknown/release/merchant_opt.wasm");

identity test;
identity alice;
identity bob;
identity merchant;


let merchant_config = record {
  order_check_duration = (60: nat64);
  order_on_hold_duration = (60: nat64);
  fee_rate = (0: float32);
  fee_to = test;
  token_allowed = vec {};
};


let args = encode fake.__init_args(
  merchant,
  merchant_config
);

let MER = install(wasm, args, null);

call MER.owner();

assert (_ : principal) == merchant;

call MER.get_merchant_info();