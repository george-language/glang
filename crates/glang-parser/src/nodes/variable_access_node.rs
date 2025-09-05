use glang_attributes::Position;
use glang_lexer::Token;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct VariableAccessNode {
    pub var_name_token: Token,
    pub pos_start: Option<Rc<Position>>,
    pub pos_end: Option<Rc<Position>>,
}

impl VariableAccessNode {
    pub fn new(var_name_token: Token) -> Self {
        Self {
            var_name_token: var_name_token.to_owned(),
            pos_start: Some(Rc::new(var_name_token.pos_start.unwrap())),
            pos_end: Some(Rc::new(var_name_token.pos_end.unwrap())),
        }
    }
}
