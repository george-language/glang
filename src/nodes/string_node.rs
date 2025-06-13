use crate::lexing::{position::Position, token::Token};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct StringNode {
    pub token: Token,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl StringNode {
    pub fn new(token: Token) -> Self {
        StringNode {
            token: token.clone(),
            pos_start: token.clone().pos_start.clone(),
            pos_end: token.clone().pos_end.clone(),
        }
    }
}

impl Display for StringNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token)
    }
}
