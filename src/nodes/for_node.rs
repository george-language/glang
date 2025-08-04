use crate::{
    lexing::{position::Position, token::Token},
    nodes::ast_node::AstNode,
};

#[derive(Debug, Clone)]
pub struct ForNode {
    pub var_name_token: Token,
    pub start_value_node: Box<AstNode>,
    pub end_value_node: Box<AstNode>,
    pub step_value_node: Option<Box<AstNode>>,
    pub body_node: Box<AstNode>,
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
    ) -> Self {
        Self {
            var_name_token: var_name_token.to_owned(),
            start_value_node,
            end_value_node,
            step_value_node,
            body_node,
            pos_start: var_name_token.pos_start,
            pos_end: var_name_token.pos_end,
        }
    }
}
