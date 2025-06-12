use crate::{
    lexing::{position::Position, token::Token},
    nodes::common_node::CommonNode,
};
use std::{any::Any, fmt::Display};

#[derive(Clone)]
pub struct FunctionDefinitionNode {
    pub var_name_token: Option<Token>,
    pub arg_name_tokens: Vec<Token>,
    pub body_node: Box<dyn CommonNode>,
    pub should_auto_return: bool,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl FunctionDefinitionNode {
    pub fn new(
        var_name_token: Option<Token>,
        arg_name_tokens: Vec<Token>,
        body_node: Box<dyn CommonNode>,
        should_auto_return: bool,
    ) -> Self {
        FunctionDefinitionNode {
            var_name_token: var_name_token.clone(),
            arg_name_tokens: arg_name_tokens.clone(),
            body_node: body_node.clone(),
            should_auto_return: should_auto_return,
            pos_start: if var_name_token.is_some() {
                var_name_token.unwrap().pos_start.clone()
            } else if arg_name_tokens.len() > 0 {
                arg_name_tokens[0].pos_start.clone()
            } else {
                body_node.position_start()
            },
            pos_end: body_node.position_end(),
        }
    }
}

impl CommonNode for FunctionDefinitionNode {
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

impl Display for FunctionDefinitionNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
