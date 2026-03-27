use crate::nodes::ast_node::AstNode;
use glang_attributes::Span;

#[derive(Debug, Clone)]
pub struct ImportNode {
    pub node_to_import: Box<AstNode>,
    pub span: Span,
}

impl ImportNode {
    pub fn new(node_to_import: Box<AstNode>) -> Self {
        Self {
            node_to_import: node_to_import.to_owned(),
            span: node_to_import.span(),
        }
    }
}
