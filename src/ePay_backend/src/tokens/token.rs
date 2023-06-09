use candid::{CandidType, Deserialize, Principal, types::principal::PrincipalError};


#[derive(CandidType, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum TokenType {
    ICRC1,
    DIP20,
    OTHER,
}

impl From<String> for TokenType {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "ICRC1" => TokenType::ICRC1,
            "DIP20" => TokenType::DIP20,
            _ => TokenType::OTHER, 
        }
    }
}

#[derive(CandidType, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug)]
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