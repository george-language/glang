use glang_attributes::Span;
use glang_lexer::Token;

#[derive(Debug, Clone)]
pub struct VariableAccessNode {
    pub var_name_token: Token,
    pub span: Span,
}

impl VariableAccessNode {
    pub fn new(var_name_token: Token) -> Self {
        Self {
            var_name_token: var_name_token.to_owned(),
            span: var_name_token.span,
        }
    }
}
