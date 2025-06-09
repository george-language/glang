use crate::{lexing::position::Position, nodes::common_node::CommonNode};
use std::fmt::Display;

#[derive(Clone)]
pub struct ListNode {
    pub element_nodes: Vec<Box<dyn CommonNode>>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl ListNode {
    pub fn new(
        element_nodes: Vec<Box<dyn CommonNode>>,
        pos_start: Option<Position>,
        pos_end: Option<Position>,
    ) -> Self {
        ListNode {
            element_nodes: element_nodes,
            pos_start: pos_start,
            pos_end: pos_end,
        }
    }
}

impl CommonNode for ListNode {
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

impl Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
