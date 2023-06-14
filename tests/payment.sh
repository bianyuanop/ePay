#!/usr/bin/ic-repl

load "info.sh";

identity merchant;
identity customer;

call MANAGER.get_user_canister();

let user_canister_principal = _?;
let USER = user_canister_principal;

call USER.register(merchant);
call MANAGER.create_merchant(merchant);

let merchant_id = _[17_724];

call MANAGER.get_merchant_by_id(merchant_id);
let merchant_cansiter_principal = _?;

let M = merchant_cansiter_principal;

call M.owner();

assert _ == merchant;

let tokens = vec { (TOKEN: principal); };
let standards = vec { "DIP20"; };
let amounts = vec { (1_000_000_00: nat); };
let payload = opt vec {};
let payload_spec = opt "string";
let acc = record { owner = customer; subaccount = opt account(customer)};

identity merchant;

call M.publish_order(tokens, standards, amounts, payload, payload_spec, acc);

let order_id = _[17_724];

identity token_minter;
call TOKEN.getMetadata();
let fee = _.fee;

call TOKEN.transfer(customer, add(amounts[0], mul(fee, (2: nat))));

identity customer;
call TOKEN.approve(M, amounts[0]);
call M.pay_order(order_id);

