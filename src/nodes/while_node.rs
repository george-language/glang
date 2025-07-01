use crate::{lexing::position::Position, nodes::ast_node::AstNode};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct WhileNode {
    pub condition_node: Box<AstNode>,
    pub body_node: Box<AstNode>,
    pub should_return_null: bool,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl WhileNode {
    pub fn new(
        condition_node: Box<AstNode>,
        body_node: Box<AstNode>,
        should_return_null: bool,
    ) -> Self {
        Self {
            condition_node: condition_node.clone(),
            body_node: body_node.clone(),
            should_return_null: should_return_null,
            pos_start: condition_node.position_start(),
            pos_end: body_node.position_end(),
        }
    }
}

impl Display for WhileNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
