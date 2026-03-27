use crate::nodes::ast_node::AstNode;
use glang_attributes::Span;
use glang_lexer::Token;

#[derive(Debug, Clone)]
pub struct BinaryOperatorNode {
    pub left_node: Box<AstNode>,
    pub op_token: Token,
    pub right_node: Box<AstNode>,
    pub span: Span,
}

impl BinaryOperatorNode {
    pub fn new(left_node: Box<AstNode>, op_token: Token, right_node: Box<AstNode>) -> Self {
        let pos_start = left_node.position_start();
        let pos_end = right_node.position_end();
        let filename = left_node.span().filename;

        Self {
            left_node,
            op_token,
            right_node,
            span: Span::new(&filename, pos_start, pos_end),
        }
    }
}
