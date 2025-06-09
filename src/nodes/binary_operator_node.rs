use crate::{
    lexing::{position::Position, token::Token},
    nodes::common_node::CommonNode,
};
use std::fmt::Display;

#[derive(Clone)]
pub struct BinaryOperatorNode {
    pub left_node: Box<dyn CommonNode>,
    pub op_token: Token,
    pub right_node: Box<dyn CommonNode>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl BinaryOperatorNode {
    pub fn new(
        left_node: Box<dyn CommonNode>,
        op_token: Token,
        right_node: Box<dyn CommonNode>,
    ) -> Self {
        let pos_start = left_node.position_start();
        let pos_end = right_node.position_end();

        BinaryOperatorNode {
            left_node: left_node,
            op_token: op_token,
            right_node: right_node,
            pos_start: pos_start,
            pos_end: pos_end,
        }
    }
}

impl CommonNode for BinaryOperatorNode {
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

impl Display for BinaryOperatorNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            self.left_node, self.op_token, self.right_node
        )
    }
}
