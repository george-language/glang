use crate::lexing::position::Position;
use crate::lexing::token::Token;
use crate::lexing::token_type::TokenType;
use crate::syntax::attributes::*;

pub struct Lexer {
    pub filename: String,
    pub text: String,
    pub position: Position,
    pub current_char: Option<char>,
}

impl Lexer {
    pub fn new(filename: String, text: String) -> Self {
        let mut lexer = Lexer {
            filename: filename.clone(),
            text: text.replace("\r\n", "\n"),
            position: Position::new(-1, 0, -1, filename, text),
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

    pub fn make_tokens(&mut self) -> (Vec<Token>, Option<Token>) {
        let mut tokens = Vec::new();

        while self.current_char != None {
            let c_char = self.current_char.clone();

            match c_char {
                Some(current_char) => {
                    if current_char == ' ' || current_char == '\t' {
                        self.advance();
                    } else if current_char == ';' || current_char == '\n' {
                        tokens.push(Token::new(
                            TokenType::TT_NEWLINE,
                            None,
                            Some(self.position.copy()),
                            None,
                        ));
                        self.advance();
                    } else if DIGITS.contains(current_char) {
                        tokens.push(self.make_number());
                    } else if LETTERS.contains(current_char) {
                        tokens.push(self.make_identifier());
                    } else if current_char == '"' {
                        // tokens.append(self.makeString())
                    } else if current_char == '+' {
                        tokens.push(Token::new(
                            TokenType::TT_PLUS,
                            None,
                            Some(self.position.copy()),
                            None,
                        ));
                        self.advance();
                    } else if current_char == '-' {
                        // tokens.append(self.makeMinusOrArrow())
                    } else if current_char == '*' {
                        tokens.push(Token::new(
                            TokenType::TT_MUL,
                            None,
                            Some(self.position.copy()),
                            None,
                        ));
                        self.advance();
                    } else if current_char == '/' {
                        tokens.push(Token::new(
                            TokenType::TT_DIV,
                            None,
                            Some(self.position.copy()),
                            None,
                        ));
                        self.advance();
                    } else if current_char == '^' {
                        tokens.push(Token::new(
                            TokenType::TT_POW,
                            None,
                            Some(self.position.copy()),
                            None,
                        ));
                        self.advance();
                    } else if current_char == '(' {
                        tokens.push(Token::new(
                            TokenType::TT_LPAREN,
                            None,
                            Some(self.position.copy()),
                            None,
                        ));
                        self.advance();
                    } else if current_char == ')' {
                        tokens.push(Token::new(
                            TokenType::TT_RPAREN,
                            None,
                            Some(self.position.copy()),
                            None,
                        ));
                        self.advance();
                    } else if current_char == '[' {
                        tokens.push(Token::new(
                            TokenType::TT_LSQUARE,
                            None,
                            Some(self.position.copy()),
                            None,
                        ));
                        self.advance();
                    } else if current_char == ']' {
                        tokens.push(Token::new(
                            TokenType::TT_RSQUARE,
                            None,
                            Some(self.position.copy()),
                            None,
                        ));
                        self.advance();
                    } else if current_char == '!' {
                        // tok, error = self.makeNotEquals()
                        //
                        // if error:
                        //     return [], error

                        // tokens.append(tok)
                    } else if current_char == '=' {
                        // tokens.append(self.makeEquals())
                    } else if current_char == '<' {
                        // tokens.append(self.makeLessThan())
                    } else if current_char == '>' {
                        // tokens.append(self.makeGreaterThan())
                    } else if current_char == ',' {
                        tokens.push(Token::new(
                            TokenType::TT_COMMA,
                            None,
                            Some(self.position.copy()),
                            None,
                        ));
                        self.advance();
                    } else {
                        let pos_start = self.position.copy();
                        let character = current_char.clone();
                        println!("Unexpected character: {:?}", character);
                        self.advance();

                        return (Vec::new(), None);
                    }
                }
                None => {}
            }
        }

        tokens.push(Token::new(
            TokenType::TT_EOF,
            None,
            Some(self.position.copy()),
            None,
        ));
        (tokens, None)
    }

    pub fn make_number(&mut self) -> Token {
        let mut num_str = String::new();
        let mut dot_count = 0;
        let pos_start = self.position.copy();

        while let Some(character) = self.current_char {
            if character.is_ascii_digit() {
                num_str.push(character);
            } else if character == '.' {
                if dot_count == 1 {
                    break;
                }
                dot_count += 1;
                num_str.push('.');
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

        Token::new(
            token_type,
            Some(num_str),
            Some(pos_start),
            Some(self.position.copy()),
        )
    }

    pub fn make_identifier(&mut self) -> Token {
        let mut id_string = String::new();
        let pos_start = self.position.copy();

        while let Some(character) = self.current_char {
            if LETTERS_DIGITS.contains(character) {
                id_string.push(character);
                self.advance();
            } else {
                break;
            }
        }

        let token_type = if KEYWORDS.contains(&id_string.as_str()) {
            TokenType::TT_KEYWORD
        } else {
            TokenType::TT_IDENTIFIER
        };

        Token::new(
            token_type,
            Some(id_string),
            Some(pos_start),
            Some(self.position.copy()),
        )
    }
}
