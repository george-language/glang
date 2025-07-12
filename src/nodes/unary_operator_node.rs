use crate::{
    lexing::{position::Position, token::Token},
    nodes::ast_node::AstNode,
};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct UnaryOperatorNode {
    pub op_token: Token,
    pub node: Box<AstNode>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl UnaryOperatorNode {
    pub fn new(op_token: Token, node: Box<AstNode>) -> Self {
        let pos_end = node.position_end();

        Self {
            op_token: op_token.to_owned(),
            node: node,
            pos_start: op_token.pos_start,
            pos_end: pos_end,
        }
    }
}

impl Display for UnaryOperatorNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.op_token, self.node)
    }
}
