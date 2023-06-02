use candid::{CandidType, Deserialize, Principal};


#[derive(CandidType, Deserialize)]
pub enum TokenType {
    ICRC1,
    DIP20,
}

#[derive(CandidType, Deserialize)]
pub struct Token {
    symbol: String,
    name: String,
    decimal: u32,
    principal: Principal,
    token_type: TokenType
}