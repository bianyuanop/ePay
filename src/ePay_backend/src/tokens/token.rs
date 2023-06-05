use candid::{CandidType, Deserialize, Principal, types::principal::PrincipalError};


#[derive(CandidType, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TokenType {
    ICRC1,
    DIP20,
    OTHER,
}

#[derive(CandidType, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
pub struct TokenInfo {
    // where the contract that manage the token
    pub principal: Principal,
    pub token_type: TokenType,
}

impl TokenInfo {
    pub fn generate_token_info(principal: Principal, token_type: TokenType) -> Self {
        Self {
            principal,
            token_type
        }
    }
}

impl Default for TokenInfo {
    fn default() -> Self {
        Self { principal: Principal::anonymous(), token_type: TokenType::OTHER } }
}