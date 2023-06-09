#!/usr/bin/ic-repl

let MER = service "bd3sg-teaaa-aaaaa-qaaba-cai";

identity test "~/.config/dfx/identity/test/identity.pem";

call MER.install_user_canister();

identity alice;
call MER.create_merchant(alice);