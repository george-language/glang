#![warn(non_camel_case_types)]

use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    TT_INT,
    TT_FLOAT,
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
    TT_AT,
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

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let text = match self {
            TokenType::TT_INT => "INT",
            TokenType::TT_FLOAT => "FLOAT",
            TokenType::TT_STR => "STRING",
            TokenType::TT_IDENTIFIER => "IDENTIFIER",
            TokenType::TT_KEYWORD => "KEYWORD",
            TokenType::TT_PLUS => "PLUS",
            TokenType::TT_MINUS => "MINUS",
            TokenType::TT_MUL => "MUL",
            TokenType::TT_DIV => "DIV",
            TokenType::TT_POW => "POW",
            TokenType::TT_MOD => "MOD",
            TokenType::TT_EQ => "EQ",
            TokenType::TT_AT => "AT",
            TokenType::TT_LPAREN => "LPAREN",
            TokenType::TT_RPAREN => "RPAREN",
            TokenType::TT_LSQUARE => "LSQUARE",
            TokenType::TT_RSQUARE => "RSQUARE",
            TokenType::TT_LBRACKET => "LBRACKET",
            TokenType::TT_RBRACKET => "RBRACKET",
            TokenType::TT_EE => "EE",
            TokenType::TT_NE => "NE",
            TokenType::TT_LT => "LT",
            TokenType::TT_GT => "GT",
            TokenType::TT_LTE => "LTE",
            TokenType::TT_GTE => "GTE",
            TokenType::TT_COMMA => "COMMA",
            TokenType::TT_ARROW => "ARROW",
            TokenType::TT_SEMICOLON => "SEMICOLON",
            TokenType::TT_EOF => "EOF",
        };
        write!(f, "{text}")
    }
}
