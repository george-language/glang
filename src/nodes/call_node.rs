use crate::{lexing::position::Position, nodes::ast_node::AstNode};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct CallNode {
    pub node_to_call: Box<AstNode>,
    pub arg_nodes: Vec<Box<AstNode>>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl CallNode {
    pub fn new(node_to_call: Box<AstNode>, arg_nodes: Vec<Box<AstNode>>) -> Self {
        CallNode {
            node_to_call: node_to_call.clone(),
            arg_nodes: arg_nodes.clone(),
            pos_start: node_to_call.position_start(),
            pos_end: if arg_nodes.len() > 0 {
                arg_nodes[arg_nodes.len() - 1].position_end()
            } else {
                node_to_call.position_end()
            },
        }
    }
}

impl Display for CallNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
