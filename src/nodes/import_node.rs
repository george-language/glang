use crate::{lexing::position::Position, nodes::ast_node::AstNode};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct ImportNode {
    pub node_to_import: Box<AstNode>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl ImportNode {
    pub fn new(node_to_import: Box<AstNode>) -> Self {
        Self {
            node_to_import: node_to_import.to_owned(),
            pos_start: node_to_import.position_start(),
            pos_end: node_to_import.position_end(),
        }
    }
}

impl Display for ImportNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
