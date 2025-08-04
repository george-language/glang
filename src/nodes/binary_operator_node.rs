use crate::{
    lexing::{position::Position, token::Token},
    nodes::ast_node::AstNode,
};

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
            left_node,
            op_token,
            right_node,
            pos_start,
            pos_end,
        }
    }
}
