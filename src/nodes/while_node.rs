use crate::{lexing::position::Position, nodes::common_node::CommonNode};
use std::fmt::Display;

#[derive(Clone)]
pub struct WhileNode {
    pub condition_node: Box<dyn CommonNode>,
    pub body_node: Box<dyn CommonNode>,
    pub should_return_null: bool,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl WhileNode {
    pub fn new(
        condition_node: Box<dyn CommonNode>,
        body_node: Box<dyn CommonNode>,
        should_return_null: bool,
    ) -> Self {
        WhileNode {
            condition_node: condition_node.clone(),
            body_node: body_node.clone(),
            should_return_null: should_return_null,
            pos_start: condition_node.position_start(),
            pos_end: body_node.position_end(),
        }
    }
}

impl CommonNode for WhileNode {
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

impl Display for WhileNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
