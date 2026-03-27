use crate::nodes::ast_node::AstNode;
use glang_attributes::Span;
use glang_lexer::Token;

#[derive(Debug, Clone)]
pub struct ConstAssignNode {
    pub const_name_token: Token,
    pub value_node: Box<AstNode>,
    pub span: Span,
}

impl ConstAssignNode {
    pub fn new(var_name_token: Token, value_node: Box<AstNode>) -> Self {
        Self {
            const_name_token: var_name_token.to_owned(),
            value_node,
            span: var_name_token.span,
        }
    }
}
