#!/usr/bin/ic-repl
load "prelude.sh";

import fake = "2vxsx-fae" as "../src/ePay_backend/src/canisters/merchant.did";
let wasm = file("../target/wasm32-unknown-unknown/release/merchant_opt.wasm");

identity alice;
let args = encode fake.__init_args(
  alice
);

let MER = install(wasm, args, null);

call MER.owner();

assert (_ : principal) == alice;

import fake2 = "2vxsx-fae" as "../icrc1_tokens/icrc1.did";
let icrc_wasm = file("../icrc1_tokens/icrc1-ledger.wasm");

let token_initialize_args = encode fake2.__init_args(
  record {
    minting_account = record {owner = alice};
    transfer_fee = 0;
    token_symbol = "TT";
    token_name = "Test Token";
    metadata = vec {};
    initial_balances = vec {};
    archive_options = record {
        num_blocks_to_archive = 2000;
        trigger_threshold = 1000;
        controller_id = alice;
    };
  }
);

let TEX = install(icrc_wasm, token_initialize_args, null);
call TEX.icrc1_symbol();
assert _ == "TEX";
call TEX.icrc1_balanceOf(account_id(alice, account(alice)));
_;