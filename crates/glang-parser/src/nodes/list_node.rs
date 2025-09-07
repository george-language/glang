use crate::nodes::ast_node::AstNode;
use glang_attributes::Position;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ListNode {
    pub element_nodes: Arc<[Box<AstNode>]>,
    pub pos_start: Option<Rc<Position>>,
    pub pos_end: Option<Rc<Position>>,
}

impl ListNode {
    pub fn new(
        element_nodes: &[Box<AstNode>],
        pos_start: Option<Rc<Position>>,
        pos_end: Option<Rc<Position>>,
    ) -> Self {
        Self {
            element_nodes: Arc::from(element_nodes),
            pos_start,
            pos_end,
        }
    }
}
