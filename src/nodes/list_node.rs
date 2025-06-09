use crate::{lexing::position::Position, nodes::common_node::CommonNode};

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

impl CommonNode for ListNode {}
