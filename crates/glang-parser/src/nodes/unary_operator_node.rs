use crate::nodes::ast_node::AstNode;
use glang_attributes::Span;
use glang_lexer::Token;

#[derive(Debug, Clone)]
pub struct UnaryOperatorNode {
    pub op_token: Token,
    pub node: Box<AstNode>,
    pub span: Span,
}

impl UnaryOperatorNode {
    pub fn new(op_token: Token, node: Box<AstNode>) -> Self {
        let pos_end = node.position_end();
        let filename = node.span().filename;

        Self {
            op_token: op_token.to_owned(),
            node,
            span: Span::new(&filename, op_token.span.start, pos_end),
        }
    }
}
