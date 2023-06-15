#!/usr/bin/bash

./scripts/build.sh
dfx canister install manager --mode='reinstall' -y

./scripts/upload_wasm.sh
./scripts/install_user_and_one_merchant.sh
./scripts/insert_supported_token_and_update_merchant.sh