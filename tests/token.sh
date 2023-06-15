#!/usr/bin/ic-repl

load "prelude.sh";
load "info.sh";

identity token_minter "~/.config/dfx/identity/test/identity.pem";

identity alice;
identity bob;
identity charlie;

call TOKEN.balanceOf(alice);
assert _ == (0: nat);

identity token_minter;

call TOKEN.transfer(alice, (1_000_000_000: nat));
call TOKEN.balanceOf(alice);
assert _ == (1_000_000_000: nat);

call TOKEN.getMetadata();
let fee = _.fee;

let fee_double = mul(fee, 2);
let fee_double_add_one = add(fee_double, 1);
let amount2approve = (sub(1_000_000_000, fee_double_add_one): nat);

let expected2approve = (999_979_999: nat);

assert amount2approve == expected2approve;

identity alice;
call TOKEN.approve(bob, amount2approve);
call TOKEN.balanceOf(alice);
assert _ == (999_990_000: nat);

call TOKEN.allowance(alice, bob);
let allowance = _;

assert allowance == (add(amount2approve, fee): nat);

call TOKEN.balanceOf(alice);
let balance = _;

assert balance == (add(amount2approve, add(fee, 1)): nat);

identity bob;

call TOKEN.transferFrom(alice, charlie, amount2approve);
call TOKEN.balanceOf(charlie);

assert _ == amount2approve;