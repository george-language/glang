use crate::{lexing::position::Position, nodes::common_node::CommonNode};
use std::fmt::Display;

#[derive(Clone)]
pub struct CallNode {
    pub node_to_call: Box<dyn CommonNode>,
    pub arg_nodes: Vec<Box<dyn CommonNode>>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl CallNode {
    pub fn new(node_to_call: Box<dyn CommonNode>, arg_nodes: Vec<Box<dyn CommonNode>>) -> Self {
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

impl CommonNode for CallNode {
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

impl Display for CallNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
