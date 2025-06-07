use crate::token::token_type::TokenType;
use std::fmt::{Display, Formatter, Result};

pub struct Token {
    token_type: TokenType,
    value: String,
    pos_start: String,
    pos_end: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String, pos_start: String, pos_end: String) -> Self {
        // pos_end.advance()
        Token {
            token_type,
            value,
            pos_start,
            pos_end,
        }
    }

    pub fn matches(&self, token_type: TokenType, value: &str) -> bool {
        self.token_type == token_type && self.value == value
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if !self.value.is_empty() {
            write!(f, "{}:{}", self.token_type, self.value)
        } else {
            write!(f, "{}", self.token_type)
        }
    }
}
