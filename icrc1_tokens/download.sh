#!/usr/bin/bash
export IC_VERSION=1612a202d030faa496e1694eed98be4179fca856
curl -o icrc1-ledger.wasm.gz "https://download.dfinity.systems/ic/$IC_VERSION/canisters/ic-icrc1-ledger.wasm.gz"
curl -o icrc1-ledger.did "https://raw.githubusercontent.com/dfinity/ic/$IC_VERSION/rs/rosetta-api/icrc1/ledger/ledger.did"
gunzip icrc1-ledger.wasm.gz