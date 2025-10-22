use glang_attributes::Position;
use glang_lexer::Token;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct NumberNode {
    pub value: f64,
    pub pos_start: Option<Rc<Position>>,
    pub pos_end: Option<Rc<Position>>,
}

impl NumberNode {
    pub fn new(token: Token) -> Self {
        Self {
            value: token.value.as_ref().unwrap().parse::<f64>().unwrap(),
            pos_start: Some(Rc::new(token.pos_start.unwrap())),
            pos_end: Some(Rc::new(token.pos_end.unwrap())),
        }
    }
}
