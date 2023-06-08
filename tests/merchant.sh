#!/usr/bin/ic-repl
load "prelude.sh";
load "dip20.sh";

import fake = "2vxsx-fae" as "../src/ePay_backend/src/canisters/merchant.did";
let wasm = file("../target/wasm32-unknown-unknown/release/merchant_opt.wasm");

identity token_minter "~/.config/dfx/identity/test/identity.pem";
identity alice;
identity bob;

identity merchant;
let args = encode fake.__init_args(
  merchant
);

let MER = install(wasm, args, null);

call MER.owner();

assert (_ : principal) == merchant;

let TOKEN = service "bkyz2-fmaaa-aaaaa-qaaaq-cai";
call TOKEN.symbol();

assert _ == "DFC";

identity token_minter;
call TOKEN.transfer(alice, 10_000_000_000);

let token_list = vec { principal "bkyz2-fmaaa-aaaaa-qaaaq-cai"; };
let token_standards = vec {"DIP20";};
let token_amount = vec {(100_000_000 : nat)};
let payload = vec {1; 2; 3; 4};
let payload_spec = "parser.v1";
let payer = account_id(alice, account(alice));

identity merchant;
call MER.publish_order(token_list, token_standards, token_amount, payload, payload_spec, payer);

identity alice;
call TOKEN.approve((MER: principal), (100_000_000: nat));
call MER.pay_order((0: nat64));

