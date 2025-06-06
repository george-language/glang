use crate::token::token_type::TokenType;

pub struct Token {
    token_type: TokenType,
    value: String,
    pos_start: String,
    pos_end: String,
}

impl Token {
    pub fn new(
        &self,
        token_type: TokenType,
        value: String,
        pos_start: String,
        pos_end: String,
    ) -> Self {
        // pos_end.advance()
        Token {
            token_type,
            value,
            pos_start,
            pos_end,
        }
    }
}
