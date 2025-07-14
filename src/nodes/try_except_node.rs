use crate::{
    lexing::{position::Position, token::Token},
    nodes::ast_node::AstNode,
};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct TryExceptNode {
    pub try_body_node: Box<AstNode>,
    pub except_body_node: Box<AstNode>,
    pub error_name_token: Token,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl TryExceptNode {
    pub fn new(
        try_body_node: Box<AstNode>,
        except_body_node: Box<AstNode>,
        error_name_token: Token,
    ) -> Self {
        Self {
            try_body_node: try_body_node.to_owned(),
            except_body_node: except_body_node.to_owned(),
            error_name_token: error_name_token,
            pos_start: try_body_node.position_start(),
            pos_end: except_body_node.position_end(),
        }
    }
}

impl Display for TryExceptNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
