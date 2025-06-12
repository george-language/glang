use crate::{
    lexing::{position::Position, token::Token},
    nodes::common_node::CommonNode,
};
use std::{any::Any, fmt::Display};

#[derive(Clone)]
pub struct StringNode {
    pub token: Token,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl StringNode {
    pub fn new(token: Token) -> Self {
        StringNode {
            token: token.clone(),
            pos_start: token.clone().pos_start.clone(),
            pos_end: token.clone().pos_end.clone(),
        }
    }
}

impl CommonNode for StringNode {
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

impl Display for StringNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token)
    }
}
