use crate::nodes::ast_node::AstNode;
use glang_attributes::Position;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct CallNode {
    pub node_to_call: Box<AstNode>,
    pub arg_nodes: Vec<Box<AstNode>>,
    pub pos_start: Option<Rc<Position>>,
    pub pos_end: Option<Rc<Position>>,
}

impl CallNode {
    pub fn new(node_to_call: Box<AstNode>, arg_nodes: Vec<Box<AstNode>>) -> Self {
        Self {
            node_to_call: node_to_call.to_owned(),
            arg_nodes: arg_nodes.to_owned(),
            pos_start: node_to_call.position_start(),
            pos_end: if !arg_nodes.is_empty() {
                arg_nodes[arg_nodes.len() - 1].position_end()
            } else {
                node_to_call.position_end()
            },
        }
    }
}
