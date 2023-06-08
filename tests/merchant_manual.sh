#!/usr/bin/ic-repl
load "prelude.sh";

identity test "~/.config/dfx/identity/test/identity.pem";
identity alice;

let MER = service "bkyz2-fmaaa-aaaaa-qaaaq-cai";
let TOKEN = service "be2us-64aaa-aaaaa-qaabq-cai";

identity test;
call TOKEN.transfer(alice, 10_000_000_000);

let token_list = vec { (TOKEN: principal); };
let token_standards = vec {"DIP20";};
let token_amount = vec {(100_000_000 : nat)};
let payload = vec {1; 2; 3; 4};
let payload_spec = "parser.v1";
let payer = account_id(alice, account(alice));

identity test;
call MER.publish_order(token_list, token_standards, token_amount, payload, payload_spec, payer);

identity alice;
call TOKEN.approve((MER: principal), (100_000_000: nat));
call MER.pay_order((0: nat64));
