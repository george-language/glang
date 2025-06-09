use crate::{lexing::position::Position, nodes::common_node::CommonNode};
use std::fmt::Display;

#[derive(Clone)]
pub struct ReturnNode {
    pub node_to_return: Box<dyn CommonNode>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl ReturnNode {
    pub fn new(
        node: Box<dyn CommonNode>,
        pos_start: Option<Position>,
        pos_end: Option<Position>,
    ) -> Self {
        ReturnNode {
            node_to_return: node,
            pos_start: pos_start,
            pos_end: pos_end,
        }
    }
}

impl CommonNode for ReturnNode {
    fn position_start(&self) -> Option<Position> {
        self.pos_start.clone()
    }

    fn position_end(&self) -> Option<Position> {
        self.pos_end.clone()
    }

    fn clone_box(&self) -> Box<dyn CommonNode> {
        Box::new(self.clone())
    }
}

impl Display for ReturnNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
