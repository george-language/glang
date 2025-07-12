use crate::lexing::{position::Position, token::Token};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct VariableAccessNode {
    pub var_name_token: Token,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl VariableAccessNode {
    pub fn new(var_name_token: Token) -> Self {
        Self {
            var_name_token: var_name_token.to_owned(),
            pos_start: var_name_token.pos_start,
            pos_end: var_name_token.pos_end,
        }
    }
}

impl Display for VariableAccessNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
