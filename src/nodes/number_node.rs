use crate::lexing::{position::Position, token::Token};

#[derive(Debug, Clone)]
pub struct NumberNode {
    pub token: Token,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl NumberNode {
    pub fn new(token: Token) -> Self {
        Self {
            token: token.to_owned(),
            pos_start: token.pos_start,
            pos_end: token.pos_end,
        }
    }
}
