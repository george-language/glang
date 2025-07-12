use crate::{
    lexing::{position::Position, token::Token},
    nodes::ast_node::AstNode,
};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct VariableAssignNode {
    pub var_name_token: Token,
    pub value_node: Box<AstNode>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl VariableAssignNode {
    pub fn new(var_name_token: Token, value_node: Box<AstNode>) -> Self {
        Self {
            var_name_token: var_name_token.to_owned(),
            value_node: value_node,
            pos_start: var_name_token.pos_start,
            pos_end: var_name_token.pos_end,
        }
    }
}

impl Display for VariableAssignNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
