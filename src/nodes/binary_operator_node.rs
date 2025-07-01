use crate::{
    lexing::{position::Position, token::Token},
    nodes::ast_node::AstNode,
};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct BinaryOperatorNode {
    pub left_node: Box<AstNode>,
    pub op_token: Token,
    pub right_node: Box<AstNode>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl BinaryOperatorNode {
    pub fn new(left_node: Box<AstNode>, op_token: Token, right_node: Box<AstNode>) -> Self {
        let pos_start = left_node.position_start();
        let pos_end = right_node.position_end();

        Self {
            left_node: left_node,
            op_token: op_token,
            right_node: right_node,
            pos_start: pos_start,
            pos_end: pos_end,
        }
    }
}

impl Display for BinaryOperatorNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            self.left_node, self.op_token, self.right_node
        )
    }
}
