use crate::token::Token;
use crate::token_type::TokenType;
use glang_attributes::Position;
use glang_attributes::StandardError;
use glang_attributes::keywords::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

pub struct Lexer {
    pub filename: String,
    pub text: String,
    pub chars: Arc<[char]>,
    pub position: Position,
    pub current_char: Option<char>,
}

impl Lexer {
    pub fn new(filename: &str, text: String) -> Self {
        let contents = text.replace("\r\n", "\n");

        let mut lexer = Self {
            filename: filename.to_string(),
            text: contents.to_string(),
            chars: contents.chars().collect::<Vec<_>>().into(),
            position: Position::new(-1, 0, -1, filename, &contents.clone()),
            current_char: None,
        };
        lexer.advance();

        lexer
    }

    fn advance(&mut self) {
        self.position.advance(self.current_char);

        if self.position.index >= 0 && (self.position.index as usize) < self.chars.len() {
            self.current_char = Some(self.chars[self.position.index as usize]);
        } else {
            self.current_char = None;
        }
    }

    pub fn make_tokens(&mut self) -> Result<Vec<Token>, StandardError> {
        let mut tokens = Vec::new();

        while let Some(current_char) = self.current_char {
            let token = match current_char {
                ' ' | '\t' | '\n' => {
                    self.advance();

                    continue;
                }
                '#' => {
                    self.skip_comment();

                    continue;
                }
                ';' => {
                    let token = Token::new(
                        TokenType::TT_SEMICOLON,
                        None,
                        Some(self.position.clone()),
                        None,
                    );

                    self.advance();

                    Some(token)
                }
                c if DIGITS.contains(c) => match self.make_number() {
                    Ok(token) => Some(token),
                    Err(error) => return Err(error),
                },
                c if LETTERS.contains(c) => Some(self.make_identifier()),
                '"' => match self.make_string() {
                    Ok(token) => Some(token),
                    Err(error) => return Err(error),
                },
                '+' => {
                    let token =
                        Token::new(TokenType::TT_PLUS, None, Some(self.position.clone()), None);

                    self.advance();

                    Some(token)
                }
                '-' => Some(self.make_minus_or_arrow()),
                '*' => {
                    let token =
                        Token::new(TokenType::TT_MUL, None, Some(self.position.clone()), None);

                    self.advance();

                    Some(token)
                }
                '/' => {
                    let token =
                        Token::new(TokenType::TT_DIV, None, Some(self.position.clone()), None);

                    self.advance();

                    Some(token)
                }
                '^' => {
                    let token =
                        Token::new(TokenType::TT_POW, None, Some(self.position.clone()), None);

                    self.advance();

                    Some(token)
                }
                '%' => {
                    let token =
                        Token::new(TokenType::TT_MOD, None, Some(self.position.clone()), None);

                    self.advance();

                    Some(token)
                }
                '(' => {
                    let token = Token::new(
                        TokenType::TT_LPAREN,
                        None,
                        Some(self.position.clone()),
                        None,
                    );

                    self.advance();

                    Some(token)
                }
                ')' => {
                    let token = Token::new(
                        TokenType::TT_RPAREN,
                        None,
                        Some(self.position.clone()),
                        None,
                    );

                    self.advance();

                    Some(token)
                }
                '[' => {
                    let token = Token::new(
                        TokenType::TT_LSQUARE,
                        None,
                        Some(self.position.clone()),
                        None,
                    );

                    self.advance();

                    Some(token)
                }
                ']' => {
                    let token = Token::new(
                        TokenType::TT_RSQUARE,
                        None,
                        Some(self.position.clone()),
                        None,
                    );

                    self.advance();

                    Some(token)
                }
                '{' => {
                    let token = Token::new(
                        TokenType::TT_LBRACKET,
                        None,
                        Some(self.position.clone()),
                        None,
                    );

                    self.advance();

                    Some(token)
                }
                '}' => {
                    let token = Token::new(
                        TokenType::TT_RBRACKET,
                        None,
                        Some(self.position.clone()),
                        None,
                    );

                    self.advance();

                    Some(token)
                }
                '!' => match self.make_not_equals() {
                    Ok(token) => Some(token),
                    Err(error) => return Err(error),
                },
                '=' => Some(self.make_equals()),
                '<' => Some(self.make_less_than()),
                '>' => Some(self.make_greater_than()),
                ',' => {
                    let token =
                        Token::new(TokenType::TT_COMMA, None, Some(self.position.clone()), None);
                    self.advance();
                    Some(token)
                }
                unknown_char => {
                    let pos_start = self.position.clone();

                    self.advance();

                    return Err(StandardError::new(
                        format!("unkown character '{unknown_char}'").as_str(),
                        Rc::new(pos_start),
                        Rc::new(self.position.clone()),
                        Some("replace this character with one known by glang"),
                    ));
                }
            };

            if let Some(t) = token {
                tokens.push(t);
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

    fn make_number(&mut self) -> Result<Token, StandardError> {
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
                    Rc::new(pos_start),
                    Rc::new(self.position.clone()),
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

    fn make_identifier(&mut self) -> Token {
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

    fn make_string(&mut self) -> Result<Token, StandardError> {
        let mut string = String::new();
        let pos_start = self.position.clone();
        let mut escape_char = false;

        self.advance();

        let mut escape_chars = HashMap::new();
        escape_chars.insert('r', '\r');
        escape_chars.insert('e', '\x1b');
        escape_chars.insert('n', '\n');
        escape_chars.insert('t', '\t');
        escape_chars.insert('\\', '\\');
        escape_chars.insert('"', '\"');

        while let Some(character) = self.current_char {
            if character == '"' && !escape_char {
                break;
            }

            if escape_char {
                if character == 'e' {
                    string.push('\x1b');
                    self.advance();

                    if self.current_char == Some('[') {
                        string.push('[');
                        self.advance();

                        while let Some(c) = self.current_char {
                            string.push(c);
                            self.advance();
                            if c == 'm' {
                                break;
                            }
                        }
                    } else {
                        return Err(StandardError::new(
                            "invalid ANSI escape sequence (expected '[')",
                            Rc::new(pos_start),
                            Rc::new(self.position.clone()),
                            None,
                        ));
                    }
                } else if let Some(replacement) = escape_chars.get(&character) {
                    string.push(*replacement);
                    self.advance();
                } else {
                    return Err(StandardError::new(
                        "invalid escape character",
                        Rc::new(pos_start),
                        Rc::new(self.position.clone()),
                        None,
                    ));
                }

                escape_char = false;

                continue;
            }

            if character == '\\' {
                escape_char = true;
            } else {
                string.push(character);
            }

            self.advance();
        }

        if self.current_char != Some('"') {
            return Err(StandardError::new(
                "unfinished string",
                Rc::new(pos_start),
                Rc::new(self.position.clone()),
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

    fn make_minus_or_arrow(&mut self) -> Token {
        let mut token_type = TokenType::TT_MINUS;
        let pos_start = self.position.clone();
        self.advance();

        if let Some(character) = self.current_char
            && character == '>'
        {
            self.advance();
            token_type = TokenType::TT_ARROW;
        }

        Token::new(
            token_type,
            None,
            Some(pos_start),
            Some(self.position.clone()),
        )
    }

    fn make_equals(&mut self) -> Token {
        let mut token_type = TokenType::TT_EQ;
        let pos_start = self.position.clone();
        self.advance();

        if let Some(character) = self.current_char
            && character == '='
        {
            self.advance();
            token_type = TokenType::TT_EE;
        }

        Token::new(
            token_type,
            None,
            Some(pos_start),
            Some(self.position.clone()),
        )
    }

    fn make_not_equals(&mut self) -> Result<Token, StandardError> {
        let pos_start = self.position.clone();
        self.advance();

        if let Some(character) = self.current_char
            && character == '='
        {
            self.advance();

            return Ok(Token::new(
                TokenType::TT_NE,
                None,
                Some(pos_start),
                Some(self.position.clone()),
            ));
        }

        self.advance();

        Err(StandardError::new(
            "expected '=' after '!'",
            Rc::new(pos_start),
            Rc::new(self.position.clone()),
            Some("add a '=' after the '!' character"),
        ))
    }

    fn make_less_than(&mut self) -> Token {
        let mut token_type = TokenType::TT_LT;
        let pos_start = self.position.clone();
        self.advance();

        if let Some(character) = self.current_char
            && character == '='
        {
            self.advance();
            token_type = TokenType::TT_LTE;
        }

        Token::new(
            token_type,
            None,
            Some(pos_start),
            Some(self.position.clone()),
        )
    }

    fn make_greater_than(&mut self) -> Token {
        let mut token_type = TokenType::TT_GT;
        let pos_start = self.position.clone();
        self.advance();

        if let Some(character) = self.current_char
            && character == '='
        {
            self.advance();
            token_type = TokenType::TT_GTE;
        }

        Token::new(
            token_type,
            None,
            Some(pos_start),
            Some(self.position.clone()),
        )
    }

    fn skip_comment(&mut self) {
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
