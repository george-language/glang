use crate::nodes::ast_node::AstNode;
use glang_attributes::Position;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ReturnNode {
    pub node_to_return: Option<Box<AstNode>>,
    pub pos_start: Option<Rc<Position>>,
    pub pos_end: Option<Rc<Position>>,
}

impl ReturnNode {
    pub fn new(
        node: Option<Box<AstNode>>,
        pos_start: Option<Rc<Position>>,
        pos_end: Option<Rc<Position>>,
    ) -> Self {
        Self {
            node_to_return: node,
            pos_start,
            pos_end,
        }
    }
}
