#!/usr/bin/ic-repl

load "prelude.sh";

identity token_minter "~/.config/dfx/identity/test/identity.pem";
identity alice;

let TOKEN = service "bkyz2-fmaaa-aaaaa-qaaaq-cai";

let wasm = file("../target/wasm32-unknown-unknown/release/dip20_opt.wasm");
let TOKEN_SERV = install(wasm, encode (), null);

identity token_minter;
call TOKEN.transfer(alice, (10_000_000_000: nat));

identity alice;
call TOKEN.approve((TOKEN_SERV: principal), (1_000_000_000: nat));

