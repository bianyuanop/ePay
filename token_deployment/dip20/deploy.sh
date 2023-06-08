cd tokens/DIP20/motoko
dfx identity use test
dfx canister create --all
dfx build
dfx canister install token --argument="(\"data:image/jpeg;base64,...\", \"DFinance Coin\", \"DFC\", 8, 10000000000000000, principal \"$(dfx identity get-principal)\", 10000)"