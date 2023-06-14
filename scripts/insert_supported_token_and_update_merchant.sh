#!/usr/bin/ic-repl

load "info.sh";

let token_principal = TOKEN;
let standard = "DIP20";

let ps = vec {token_principal; };
let ss = vec {standard; };

let MANAGER = service "bd3sg-teaaa-aaaaa-qaaba-cai";

identity test "~/.config/dfx/identity/test/identity.pem";

call MANAGER.update_allowed_token_list(ps, ss);
call MANAGER.update_all_merchant_allowed_tokens();
