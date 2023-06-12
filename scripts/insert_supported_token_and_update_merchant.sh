#!/usr/bin/ic-repl

let token_principal = principal "d6g4o-amaaa-aaaaa-qaaoq-cai";
let standard = "DIP20";

let ps = vec {token_principal; };
let ss = vec {standard; };

let MANAGER = service "bd3sg-teaaa-aaaaa-qaaba-cai";

identity test "~/.config/dfx/identity/test/identity.pem";

call MANAGER.update_allowed_token_list(ps, ss);
call MANAGER.update_all_merchant_allowed_tokens();
