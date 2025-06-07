use crate::lexing::position::Position;
use crate::lexing::token::Token;
use crate::syntax::attributes::*;

pub struct Lexer {
    pub filename: String,
    pub text: String,
    pub position: Position,
    pub current_char: char,
}

impl Lexer {
    pub fn new(filename: String, text: String) -> Self {
        let mut lexer = Lexer {
            filename: filename.clone(),
            text: text.clone(),
            position: Position::new(-1, 0, -1, filename.clone(), text.clone()),
            current_char: ' ',
        };
        lexer.advance();

        lexer
    }

    pub fn advance(&mut self) {
        self.position.advance(self.current_char);

        if self.position.index >= 0 && (self.position.index as usize) < self.text.len() {
            self.current_char = self
                .text
                .chars()
                .nth(self.position.index as usize)
                .unwrap_or(' ');
        } else {
            self.current_char = ' ';
        }
    }

    pub fn make_tokens() {}
}
