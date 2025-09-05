use glang_attributes::Position;
use glang_lexer::Token;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct StringNode {
    pub token: Token,
    pub pos_start: Option<Rc<Position>>,
    pub pos_end: Option<Rc<Position>>,
}

impl StringNode {
    pub fn new(token: Token) -> Self {
        Self {
            token: token.to_owned(),
            pos_start: Some(Rc::new(token.pos_start.unwrap())),
            pos_end: Some(Rc::new(token.pos_end.unwrap())),
        }
    }
}
