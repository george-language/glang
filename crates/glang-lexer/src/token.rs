use glang_attributes::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TokenType {
    TT_NUM,
    TT_STR,
    TT_IDENTIFIER,
    TT_KEYWORD,
    TT_PLUS,
    TT_MINUS,
    TT_MUL,
    TT_DIV,
    TT_POW,
    TT_MOD,
    TT_EQ,
    TT_LPAREN,
    TT_RPAREN,
    TT_LSQUARE,
    TT_RSQUARE,
    TT_LBRACKET,
    TT_RBRACKET,
    TT_EE,
    TT_NE,
    TT_LT,
    TT_GT,
    TT_LTE,
    TT_GTE,
    TT_COMMA,
    TT_ARROW,
    TT_SEMICOLON,
    TT_EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
    pub span: Span,
}

impl Token {
    pub fn new(token_type: TokenType, value: Option<String>, span: Span) -> Self {
        Self {
            token_type,
            value,
            span,
        }
    }

    pub fn matches(&self, token_type: TokenType, value: &str) -> bool {
        if self.value.is_some() {
            self.token_type == token_type && self.value.as_ref().unwrap() == value
        } else {
            false
        }
    }
}
