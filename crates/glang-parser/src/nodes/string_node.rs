use glang_attributes::Span;
use glang_lexer::Token;

#[derive(Debug, Clone)]
pub struct StringNode {
    pub token: Token,
    pub span: Span,
}

impl StringNode {
    pub fn new(token: Token) -> Self {
        Self {
            token: token.to_owned(),
            span: token.span,
        }
    }
}
