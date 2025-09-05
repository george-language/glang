use crate::nodes::ast_node::AstNode;
use glang_attributes::Position;
use glang_lexer::Token;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct UnaryOperatorNode {
    pub op_token: Token,
    pub node: Box<AstNode>,
    pub pos_start: Option<Rc<Position>>,
    pub pos_end: Option<Rc<Position>>,
}

impl UnaryOperatorNode {
    pub fn new(op_token: Token, node: Box<AstNode>) -> Self {
        let pos_end = node.position_end();

        Self {
            op_token: op_token.to_owned(),
            node,
            pos_start: Some(Rc::new(op_token.pos_start.unwrap())),
            pos_end,
        }
    }
}
