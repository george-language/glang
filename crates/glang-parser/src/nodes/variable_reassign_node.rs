use crate::nodes::ast_node::AstNode;
use glang_attributes::Span;
use glang_lexer::Token;

#[derive(Debug, Clone)]
pub struct VariableRessignNode {
    pub var_name_token: Token,
    pub value_node: Box<AstNode>,
    pub span: Span,
}

impl VariableRessignNode {
    pub fn new(var_name_token: Token, value_node: Box<AstNode>) -> Self {
        Self {
            var_name_token: var_name_token.to_owned(),
            value_node,
            span: var_name_token.span,
        }
    }
}
