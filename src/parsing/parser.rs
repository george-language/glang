use crate::lexing::token::Token;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub token_index: isize,
    pub current_token: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Parser {
            tokens: tokens,
            token_index: -1,
            current_token: None,
        };
        parser.advance();

        parser
    }

    pub fn advance(&mut self) -> Option<Token> {
        self.token_index += 1;
        self.update_current_token();

        self.current_token.clone()
    }

    pub fn reverse(&mut self, amount: usize) -> Option<Token> {
        self.token_index -= amount as isize;
        self.update_current_token();

        self.current_token.clone()
    }

    pub fn update_current_token(&mut self) {
        if self.token_index >= 0 && self.token_index < self.tokens.len() as isize {
            self.current_token = Some(self.tokens[self.token_index as usize].clone());
        }
    }

    pub fn parse(&self) {}
}
