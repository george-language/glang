use crate::nodes::ast_node::AstNode;
use glang_attributes::Span;

#[derive(Debug, Clone)]
pub struct CallNode {
    pub node_to_call: Box<AstNode>,
    pub arg_nodes: Vec<Box<AstNode>>,
    pub span: Span,
}

impl CallNode {
    pub fn new(node_to_call: Box<AstNode>, arg_nodes: Vec<Box<AstNode>>) -> Self {
        Self {
            node_to_call: node_to_call.to_owned(),
            arg_nodes: arg_nodes.to_owned(),
            span: Span::new(
                &node_to_call.span().filename,
                node_to_call.position_start(),
                if !arg_nodes.is_empty() {
                    arg_nodes[arg_nodes.len() - 1].position_end()
                } else {
                    node_to_call.position_end()
                },
            ),
        }
    }
}
