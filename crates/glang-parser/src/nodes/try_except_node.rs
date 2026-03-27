use crate::nodes::ast_node::AstNode;
use glang_attributes::{Position, Span};
use glang_lexer::Token;

#[derive(Debug, Clone)]
pub struct TryExceptNode {
    pub try_body_node: Box<AstNode>,
    pub except_body_node: Box<AstNode>,
    pub error_name_token: Token,
    pub span: Span,
}

impl TryExceptNode {
    pub fn new(
        try_body_node: Box<AstNode>,
        except_body_node: Box<AstNode>,
        error_name_token: Token,
    ) -> Self {
        Self {
            try_body_node: try_body_node.to_owned(),
            except_body_node: except_body_node.to_owned(),
            error_name_token,
            span: Span::new(
                &try_body_node.span().filename,
                try_body_node.position_start(),
                except_body_node.position_end(),
            ),
        }
    }
}
