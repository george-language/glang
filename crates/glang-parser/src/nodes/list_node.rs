use crate::nodes::ast_node::AstNode;
use glang_attributes::Position;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ListNode {
    pub element_nodes: Arc<[Box<AstNode>]>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl ListNode {
    pub fn new(
        element_nodes: &[Box<AstNode>],
        pos_start: Option<Position>,
        pos_end: Option<Position>,
    ) -> Self {
        Self {
            element_nodes: Arc::from(element_nodes),
            pos_start,
            pos_end,
        }
    }
}
