use crate::nodes::ast_node::AstNode;
use glang_attributes::Span;

#[derive(Debug, Clone)]
pub struct WhileNode {
    pub condition_node: Box<AstNode>,
    pub body_node: Box<AstNode>,
    pub span: Span,
}

impl WhileNode {
    pub fn new(condition_node: Box<AstNode>, body_node: Box<AstNode>) -> Self {
        Self {
            condition_node: condition_node.clone(),
            body_node: body_node.clone(),
            span: Span::new(
                &condition_node.span().filename,
                condition_node.position_start(),
                body_node.position_end(),
            ),
        }
    }
}
