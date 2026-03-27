use crate::nodes::ast_node::AstNode;
use glang_attributes::Span;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ListNode {
    pub element_nodes: Rc<[Box<AstNode>]>,
    pub span: Span,
}

impl ListNode {
    pub fn new(element_nodes: &[Box<AstNode>], span: Span) -> Self {
        Self {
            element_nodes: Rc::from(element_nodes),
            span,
        }
    }
}
