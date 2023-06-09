#!/usr/bin/bash

dfx start --background
dfx canister create manager
dfx canister create merchant
dfx canister create user

dfx build user
dfx build manager
dfx build merchant

cargo run --bin user > ./src/ePay_backend/src/canisters/user.did
cargo run --bin manager > ./src/ePay_backend/src/canisters/manager.did
cargo run --bin merchant > ./src/ePay_backend/src/canisters/merchant.did
