use crate::{lexing::position::Position, nodes::ast_node::AstNode};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct ReturnNode {
    pub node_to_return: Option<Box<AstNode>>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl ReturnNode {
    pub fn new(
        node: Option<Box<AstNode>>,
        pos_start: Option<Position>,
        pos_end: Option<Position>,
    ) -> Self {
        ReturnNode {
            node_to_return: node,
            pos_start: pos_start,
            pos_end: pos_end,
        }
    }
}

impl Display for ReturnNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
