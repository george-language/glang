use crate::{
    lexing::{position::Position, token::Token},
    nodes::common_node::CommonNode,
};
use std::fmt::Display;

#[derive(Clone)]
pub struct UnaryOperatorNode {
    pub op_token: Token,
    pub node: Box<dyn CommonNode>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl UnaryOperatorNode {
    pub fn new(op_token: Token, node: Box<dyn CommonNode>) -> Self {
        let pos_end = node.position_end();

        UnaryOperatorNode {
            op_token: op_token.clone(),
            node: node,
            pos_start: op_token.pos_start.clone(),
            pos_end: pos_end,
        }
    }
}

impl CommonNode for UnaryOperatorNode {
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

impl Display for UnaryOperatorNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.op_token, self.node)
    }
}
