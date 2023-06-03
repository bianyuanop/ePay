use candid::{CandidType, Deserialize, Principal};


#[derive(CandidType, Deserialize)]
pub enum TokenType {
    ICRC1,
    DIP20,
}

#[derive(CandidType, Deserialize)]
pub struct TokenInfo {
    name: String,
    symbol: String,
    decimal: u8,
    principal: Principal,
    token_type: TokenType,
}