use crate::{lexing::position::Position, nodes::common_node::CommonNode};
use std::{any::Any, fmt::Display};

#[derive(Clone)]
pub struct BreakNode {
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl BreakNode {
    pub fn new(pos_start: Option<Position>, pos_end: Option<Position>) -> Self {
        BreakNode {
            pos_start: pos_start.clone(),
            pos_end: pos_end.clone(),
        }
    }
}

impl CommonNode for BreakNode {
    fn position_start(&self) -> Option<Position> {
        self.pos_start.clone()
    }

    fn position_end(&self) -> Option<Position> {
        self.pos_end.clone()
    }

    fn clone_box(&self) -> Box<dyn CommonNode> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        return self;
    }
}

impl Display for BreakNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
