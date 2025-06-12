use crate::{
    lexing::{position::Position, token::Token},
    nodes::common_node::CommonNode,
};
use std::{any::Any, fmt::Display};

#[derive(Clone)]
pub struct ForNode {
    pub var_name_token: Token,
    pub start_value_node: Box<dyn CommonNode>,
    pub end_value_node: Box<dyn CommonNode>,
    pub step_value_node: Option<Box<dyn CommonNode>>,
    pub body_node: Box<dyn CommonNode>,
    pub should_return_null: bool,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl ForNode {
    pub fn new(
        var_name_token: Token,
        start_value_node: Box<dyn CommonNode>,
        end_value_node: Box<dyn CommonNode>,
        step_value_node: Option<Box<dyn CommonNode>>,
        body_node: Box<dyn CommonNode>,
        should_return_null: bool,
    ) -> Self {
        let var_name_token = var_name_token.clone();

        ForNode {
            var_name_token: var_name_token.clone(),
            start_value_node: start_value_node,
            end_value_node: end_value_node,
            step_value_node: step_value_node,
            body_node: body_node,
            should_return_null: should_return_null,
            pos_start: var_name_token.pos_start,
            pos_end: var_name_token.pos_end,
        }
    }
}

impl CommonNode for ForNode {
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

impl Display for ForNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
