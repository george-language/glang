use crate::nodes::ast_node::AstNode;
use glang_attributes::Position;
use glang_lexer::Token;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ConstAssignNode {
    pub const_name_token: Token,
    pub value_node: Box<AstNode>,
    pub pos_start: Option<Rc<Position>>,
    pub pos_end: Option<Rc<Position>>,
}

impl ConstAssignNode {
    pub fn new(var_name_token: Token, value_node: Box<AstNode>) -> Self {
        Self {
            const_name_token: var_name_token.to_owned(),
            value_node,
            pos_start: Some(Rc::new(var_name_token.pos_start.unwrap())),
            pos_end: Some(Rc::new(var_name_token.pos_end.unwrap())),
        }
    }
}
