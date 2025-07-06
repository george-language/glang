use crate::errors::standard_error::StandardError;
use crate::lexing::position::Position;
use crate::lexing::token::Token;
use crate::lexing::token_type::TokenType;
use crate::syntax::attributes::*;
use std::collections::HashMap;

pub struct Lexer {
    pub filename: String,
    pub text: String,
    pub position: Position,
    pub current_char: Option<char>,
}

impl Lexer {
    pub fn new(filename: &str, text: String) -> Self {
        let contents = text.replace("\r\n", "\n");

        let mut lexer = Self {
            filename: filename.to_string(),
            text: contents.to_string(),
            position: Position::new(-1, 0, -1, filename, &contents.clone()),
            current_char: None,
        };
        lexer.advance();

        lexer
    }

    pub fn advance(&mut self) {
        self.position.advance(self.current_char);

        if self.position.index >= 0 && (self.position.index as usize) < self.text.len() {
            self.current_char = Some(
                self.text
                    .chars()
                    .nth(self.position.index as usize)
                    .unwrap_or(' '),
            );
        } else {
            self.current_char = None;
        }
    }

    pub fn make_tokens(&mut self) -> Result<Vec<Token>, StandardError> {
        let mut tokens = Vec::new();

        while self.current_char != None {
            if let Some(current_char) = self.current_char.clone() {
                if current_char == ' ' || current_char == '\t' {
                    self.advance();
                } else if current_char == '#' {
                    self.skip_comment();
                } else if current_char == ';' || current_char == '\n' {
                    tokens.push(Token::new(
                        TokenType::TT_NEWLINE,
                        None,
                        Some(self.position.clone()),
                        None,
                    ));
                    self.advance();
                } else if DIGITS.contains(current_char) {
                    let result = self.make_number();

                    match result {
                        Ok(token) => {
                            tokens.push(token);
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                } else if LETTERS.contains(current_char) {
                    tokens.push(self.make_identifier());
                } else if current_char == '"' {
                    let result = self.make_string();

                    match result {
                        Ok(token) => {
                            tokens.push(token);
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                } else if current_char == '+' {
                    tokens.push(Token::new(
                        TokenType::TT_PLUS,
                        None,
                        Some(self.position.clone()),
                        None,
                    ));
                    self.advance();
                } else if current_char == '-' {
                    tokens.push(self.make_minus_or_arrow());
                } else if current_char == '*' {
                    tokens.push(Token::new(
                        TokenType::TT_MUL,
                        None,
                        Some(self.position.clone()),
                        None,
                    ));
                    self.advance();
                } else if current_char == '/' {
                    tokens.push(Token::new(
                        TokenType::TT_DIV,
                        None,
                        Some(self.position.clone()),
                        None,
                    ));
                    self.advance();
                } else if current_char == '^' {
                    tokens.push(Token::new(
                        TokenType::TT_POW,
                        None,
                        Some(self.position.clone()),
                        None,
                    ));
                    self.advance();
                } else if current_char == '%' {
                    tokens.push(Token::new(
                        TokenType::TT_MOD,
                        None,
                        Some(self.position.clone()),
                        None,
                    ));
                    self.advance();
                } else if current_char == '(' {
                    tokens.push(Token::new(
                        TokenType::TT_LPAREN,
                        None,
                        Some(self.position.clone()),
                        None,
                    ));
                    self.advance();
                } else if current_char == ')' {
                    tokens.push(Token::new(
                        TokenType::TT_RPAREN,
                        None,
                        Some(self.position.clone()),
                        None,
                    ));
                    self.advance();
                } else if current_char == '[' {
                    tokens.push(Token::new(
                        TokenType::TT_LSQUARE,
                        None,
                        Some(self.position.clone()),
                        None,
                    ));
                    self.advance();
                } else if current_char == ']' {
                    tokens.push(Token::new(
                        TokenType::TT_RSQUARE,
                        None,
                        Some(self.position.clone()),
                        None,
                    ));
                    self.advance();
                } else if current_char == '{' {
                    tokens.push(Token::new(
                        TokenType::TT_LBRACKET,
                        None,
                        Some(self.position.clone()),
                        None,
                    ));
                    self.advance();
                } else if current_char == '}' {
                    tokens.push(Token::new(
                        TokenType::TT_RBRACKET,
                        None,
                        Some(self.position.clone()),
                        None,
                    ));
                    self.advance();
                } else if current_char == '!' {
                    let result = self.make_not_equals();

                    match result {
                        Ok(token) => {
                            tokens.push(token);
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                } else if current_char == '=' {
                    tokens.push(self.make_equals());
                } else if current_char == '<' {
                    tokens.push(self.make_less_than())
                } else if current_char == '>' {
                    tokens.push(self.make_greater_than())
                } else if current_char == ',' {
                    tokens.push(Token::new(
                        TokenType::TT_COMMA,
                        None,
                        Some(self.position.clone()),
                        None,
                    ));
                    self.advance();
                } else {
                    let pos_start = self.position.clone();
                    let character = current_char.clone();
                    self.advance();

                    return Err(StandardError::new(
                        format!("unkown character '{}'", character).as_str(),
                        pos_start,
                        self.position.clone(),
                        Some("replace this character with one known by glang"),
                    ));
                }
            }
        }

        tokens.push(Token::new(
            TokenType::TT_EOF,
            None,
            Some(self.position.clone()),
            None,
        ));

        Ok(tokens)
    }

    pub fn make_number(&mut self) -> Result<Token, StandardError> {
        let mut num_str = String::new();
        let mut dot_count = 0;
        let pos_start = self.position.clone();

        while let Some(character) = self.current_char {
            if character.is_ascii_digit() {
                num_str.push(character);
            } else if character == '.' {
                if dot_count == 1 {
                    break;
                }
                dot_count += 1;
                num_str.push('.');
            } else if LETTERS.contains(character) {
                return Err(StandardError::new(
                    "object names cannot start with numerical values",
                    pos_start,
                    self.position.clone(),
                    None,
                ));
            } else {
                break;
            }

            self.advance();
        }

        let token_type = if dot_count == 0 {
            TokenType::TT_INT
        } else {
            TokenType::TT_FLOAT
        };

        Ok(Token::new(
            token_type,
            Some(num_str),
            Some(pos_start),
            Some(self.position.clone()),
        ))
    }

    pub fn make_identifier(&mut self) -> Token {
        let mut id_string = String::new();
        let pos_start = self.position.clone();

        while let Some(character) = self.current_char {
            if LETTERS_DIGITS.contains(character) {
                id_string.push(character);

                self.advance();
            } else {
                break;
            }
        }

        let pos_end = self.position.clone();

        let token_type = if KEYWORDS.contains(&id_string.as_str()) {
            TokenType::TT_KEYWORD
        } else {
            TokenType::TT_IDENTIFIER
        };

        Token::new(token_type, Some(id_string), Some(pos_start), Some(pos_end))
    }

    pub fn make_string(&mut self) -> Result<Token, StandardError> {
        let mut string = String::new();
        let pos_start = self.position.clone();
        let mut escape_char = false;

        self.advance();

        let mut escape_chars = HashMap::new();
        escape_chars.insert('n', '\n');
        escape_chars.insert('r', '\r');
        escape_chars.insert('t', '\t');
        escape_chars.insert('"', '\"');

        while let Some(character) = self.current_char {
            if character == '"' && !escape_char {
                break;
            }

            if escape_char {
                match escape_chars.get(&character) {
                    Some(char) => {
                        string.push(*char);
                    }
                    None => {
                        return Err(StandardError::new(
                            "invalid escape character",
                            pos_start.clone(),
                            self.position.clone(),
                            None,
                        ));
                    }
                }

                escape_char = false;
            } else if character == '\\' {
                escape_char = true;
            } else {
                string.push(character);
            }

            self.advance();
        }

        if self.current_char != Some('"') {
            return Err(StandardError::new(
                "unterminated string literal",
                pos_start,
                self.position.clone(),
                Some("add a '\"' at the end of the string to close it"),
            ));
        }

        self.advance();

        let pos_end = self.position.clone();

        Ok(Token::new(
            TokenType::TT_STR,
            Some(string),
            Some(pos_start),
            Some(pos_end),
        ))
    }

    pub fn make_minus_or_arrow(&mut self) -> Token {
        let mut token_type = TokenType::TT_MINUS;
        let pos_start = self.position.clone();
        self.advance();

        if let Some(character) = self.current_char {
            if character == '>' {
                self.advance();
                token_type = TokenType::TT_ARROW;
            }
        }

        Token::new(
            token_type,
            None,
            Some(pos_start),
            Some(self.position.clone()),
        )
    }

    pub fn make_equals(&mut self) -> Token {
        let mut token_type = TokenType::TT_EQ;
        let pos_start = self.position.clone();
        self.advance();

        if let Some(character) = self.current_char {
            if character == '=' {
                self.advance();
                token_type = TokenType::TT_EE;
            }
        }

        Token::new(
            token_type,
            None,
            Some(pos_start),
            Some(self.position.clone()),
        )
    }

    pub fn make_not_equals(&mut self) -> Result<Token, StandardError> {
        let pos_start = self.position.clone();
        self.advance();

        if let Some(character) = self.current_char {
            if character == '=' {
                self.advance();

                return Ok(Token::new(
                    TokenType::TT_NE,
                    None,
                    Some(pos_start),
                    Some(self.position.clone()),
                ));
            }
        }

        self.advance();

        return Err(StandardError::new(
            "expected '=' after '!'",
            pos_start,
            self.position.clone(),
            Some("add a '=' after the '!' character"),
        ));
    }

    pub fn make_less_than(&mut self) -> Token {
        let mut token_type = TokenType::TT_LT;
        let pos_start = self.position.clone();
        self.advance();

        if let Some(character) = self.current_char {
            if character == '=' {
                self.advance();
                token_type = TokenType::TT_LTE;
            }
        }

        Token::new(
            token_type,
            None,
            Some(pos_start),
            Some(self.position.clone()),
        )
    }

    pub fn make_greater_than(&mut self) -> Token {
        let mut token_type = TokenType::TT_GT;
        let pos_start = self.position.clone();
        self.advance();

        if let Some(character) = self.current_char {
            if character == '=' {
                self.advance();
                token_type = TokenType::TT_GTE;
            }
        }

        Token::new(
            token_type,
            None,
            Some(pos_start),
            Some(self.position.clone()),
        )
    }

    pub fn skip_comment(&mut self) {
        self.advance();

        while let Some(character) = self.current_char {
            if character != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }
}
