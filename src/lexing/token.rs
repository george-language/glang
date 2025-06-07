use crate::lexing::{position::Position, token_type::TokenType};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    value: Option<String>,
    pos_start: Option<Position>,
    pos_end: Option<Position>,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        value: Option<String>,
        pos_start: Option<Position>,
        pos_end: Option<Position>,
    ) -> Self {
        let start = pos_start.clone();
        let mut end = pos_end;

        if let Some(s) = &start {
            let mut advanced = s.clone();

            advanced.advance(None);
            end = Some(advanced);
        }

        Token {
            token_type: token_type,
            value: value,
            pos_start: start,
            pos_end: end,
        }
    }

    pub fn matches(&self, token_type: TokenType, value: Option<String>) -> bool {
        self.token_type == token_type && self.value == value
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some(ref value) = self.value {
            if !value.is_empty() {
                write!(f, "{}:{}", self.token_type, value)
            } else {
                write!(f, "{}", self.token_type)
            }
        } else {
            write!(f, "{}", self.token_type)
        }
    }
}
