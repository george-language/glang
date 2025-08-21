use crate::nodes::ast_node::AstNode;
use glang_attributes::Position;
use glang_lexer::Token;

#[derive(Debug, Clone)]
pub struct UnaryOperatorNode {
    pub op_token: Token,
    pub node: Box<AstNode>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl UnaryOperatorNode {
    pub fn new(op_token: Token, node: Box<AstNode>) -> Self {
        let pos_end = node.position_end();

        Self {
            op_token: op_token.to_owned(),
            node,
            pos_start: op_token.pos_start,
            pos_end,
        }
    }
}
