use candid::{CandidType, Deserialize, Principal};


#[derive(CandidType, Deserialize)]
pub enum TokenType {
    ICRC1,
    DIP20,
}

#[derive(CandidType, Deserialize)]
pub struct TokenInfo {
    // where the contract that manage the token
    principal: Principal,
    token_type: TokenType,
}