use crate::nodes::ast_node::AstNode;
use glang_attributes::Span;
use glang_lexer::Token;

#[derive(Debug, Clone)]
pub struct CallNode {
    pub node_to_call: Box<AstNode>,
    pub arg_nodes: Vec<Box<AstNode>>,
    pub span: Span,
}

impl CallNode {
    pub fn new(
        node_to_call: Box<AstNode>,
        arg_nodes: Vec<Box<AstNode>>,
        closing_bracket: Token,
    ) -> Self {
        Self {
            node_to_call: node_to_call.to_owned(),
            arg_nodes: arg_nodes.to_owned(),
            span: Span::new(
                &node_to_call.span().filename,
                node_to_call.position_start(),
                closing_bracket.span.end,
            ),
        }
    }
}
