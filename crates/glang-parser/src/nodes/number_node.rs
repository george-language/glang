use glang_attributes::Span;
use glang_lexer::Token;

#[derive(Debug, Clone)]
pub struct NumberNode {
    pub value: f64,
    pub span: Span,
}

impl NumberNode {
    pub fn new(token: Token) -> Self {
        Self {
            value: token.value.as_ref().unwrap().parse::<f64>().unwrap(),
            span: token.span,
        }
    }
}
