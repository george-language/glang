use crate::nodes::ast_node::AstNode;
use glang_attributes::Position;
use glang_lexer::Token;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct TryExceptNode {
    pub try_body_node: Box<AstNode>,
    pub except_body_node: Box<AstNode>,
    pub error_name_token: Token,
    pub pos_start: Option<Rc<Position>>,
    pub pos_end: Option<Rc<Position>>,
}

impl TryExceptNode {
    pub fn new(
        try_body_node: Box<AstNode>,
        except_body_node: Box<AstNode>,
        error_name_token: Token,
    ) -> Self {
        Self {
            try_body_node: try_body_node.to_owned(),
            except_body_node: except_body_node.to_owned(),
            error_name_token,
            pos_start: try_body_node.position_start(),
            pos_end: except_body_node.position_end(),
        }
    }
}
