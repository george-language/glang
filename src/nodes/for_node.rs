use crate::{
    lexing::{position::Position, token::Token},
    nodes::ast_node::AstNode,
};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct ForNode {
    pub var_name_token: Token,
    pub start_value_node: Box<AstNode>,
    pub end_value_node: Box<AstNode>,
    pub step_value_node: Option<Box<AstNode>>,
    pub body_node: Box<AstNode>,
    pub should_return_null: bool,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl ForNode {
    pub fn new(
        var_name_token: Token,
        start_value_node: Box<AstNode>,
        end_value_node: Box<AstNode>,
        step_value_node: Option<Box<AstNode>>,
        body_node: Box<AstNode>,
        should_return_null: bool,
    ) -> Self {
        let var_name_token = var_name_token.clone();

        Self {
            var_name_token: var_name_token.clone(),
            start_value_node: start_value_node,
            end_value_node: end_value_node,
            step_value_node: step_value_node,
            body_node: body_node,
            should_return_null: should_return_null,
            pos_start: var_name_token.pos_start,
            pos_end: var_name_token.pos_end,
        }
    }
}

impl Display for ForNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
