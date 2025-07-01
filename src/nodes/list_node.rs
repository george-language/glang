use crate::{lexing::position::Position, nodes::ast_node::AstNode};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct ListNode {
    pub element_nodes: Vec<Option<Box<AstNode>>>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl ListNode {
    pub fn new(
        element_nodes: Vec<Option<Box<AstNode>>>,
        pos_start: Option<Position>,
        pos_end: Option<Position>,
    ) -> Self {
        Self {
            element_nodes: element_nodes,
            pos_start: pos_start,
            pos_end: pos_end,
        }
    }
}

impl Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
