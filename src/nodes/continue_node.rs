use crate::{lexing::position::Position, nodes::common_node::CommonNode};
use std::fmt::Display;

#[derive(Clone)]
pub struct ContinueNode {
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl ContinueNode {
    pub fn new(pos_start: Option<Position>, pos_end: Option<Position>) -> Self {
        ContinueNode {
            pos_start: pos_start.clone(),
            pos_end: pos_end.clone(),
        }
    }
}

impl CommonNode for ContinueNode {
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

impl Display for ContinueNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
