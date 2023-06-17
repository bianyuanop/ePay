#!/usr/bin/ic-repl

load "../info.sh";

identity test "~/.config/dfx/identity/test/identity.pem";

call MANAGER.get_user_canister();

let USER = _?;

call USER.register(test);
call MANAGER.create_merchant(test);

let merchant_id = _[17_724];

call MANAGER.get_merchant_by_id(merchant_id);

let M = _?;

call M.owner();
assert _ == test;

call M.set_notifer("127.0.0.1:9000", "https://127.0.0.1:9000");

let tokens = vec { (TOKEN: principal); };
let standards = vec { "DIP20"; };
let amounts = vec { (1_000_000_00: nat); };
let payload = opt vec {};
let payload_spec = opt "string";
let acc = record { owner = test; subaccount = opt account(test)};

call M.publish_order(tokens, standards, amounts, payload, payload_spec, acc);

let order_id = _[17_724];

call TOKEN.approve(M, amounts[0]);
call M.pay_order(order_id);
