use crate::{
    lexing::{position::Position, token::Token},
    nodes::ast_node::AstNode,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct FunctionDefinitionNode {
    pub var_name_token: Option<Token>,
    pub arg_name_tokens: Arc<[Token]>,
    pub body_node: Box<AstNode>,
    pub should_auto_return: bool,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl FunctionDefinitionNode {
    pub fn new(
        var_name_token: Option<Token>,
        arg_name_tokens: &[Token],
        body_node: Box<AstNode>,
        should_auto_return: bool,
    ) -> Self {
        Self {
            var_name_token: var_name_token.to_owned(),
            arg_name_tokens: Arc::from(arg_name_tokens),
            body_node: body_node.to_owned(),
            should_auto_return,
            pos_start: if var_name_token.is_some() {
                var_name_token.unwrap().pos_end
            } else if !arg_name_tokens.is_empty() {
                arg_name_tokens[0].pos_start.to_owned()
            } else {
                body_node.position_start()
            },
            pos_end: body_node.position_end(),
        }
    }
}
