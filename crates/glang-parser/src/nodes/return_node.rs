use crate::nodes::ast_node::AstNode;
use glang_attributes::Span;

#[derive(Debug, Clone)]
pub struct ReturnNode {
    pub node_to_return: Option<Box<AstNode>>,
    pub span: Span,
}

impl ReturnNode {
    pub fn new(node: Option<Box<AstNode>>, span: Span) -> Self {
        Self {
            node_to_return: node,
            span,
        }
    }
}
