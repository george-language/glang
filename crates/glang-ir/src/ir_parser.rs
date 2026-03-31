use crate::{IrBinaryOperatorNode, IrBreakNode, IrImportNode, IrListNode, IrNode, IrSpan};
use glang_attributes::StandardError;
use glang_parser::AstNode;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IrDescription {
    pub files: HashMap<(String, String), IrNode>, // (filename, contents), root node
}

impl IrDescription {
    pub fn new(files: HashMap<(String, String), IrNode>) -> Self {
        Self { files }
    }
}

pub fn parse_node_into_ir(node: &AstNode) -> Result<IrDescription, StandardError> {
    let mut root_node = IrListNode::new(Vec::new(), IrSpan::from_span(node.span()));
    let desc = IrDescription::new(HashMap::new());

    // root_node
    //     .element_nodes
    //     .push(parse_node(node).ok().unwrap().0);

    Ok(desc)
}

fn parse_node(node: &AstNode) -> Result<IrNode, StandardError> {
    // let contents = match fs::read_to_string(&node.span().filename) {
    //     Ok(c) => c,
    //     Err(_) => {
    //         return Err(StandardError::new("file does not exist", node.span(), None));
    //     }
    // };

    // match node {
    //     AstNode::Import(n) => IrNode::Import(IrImportNode::new(
    //         match parse_node(&node_to_import) {
    //             Ok(import) => Box::new(import),
    //             Err(e) => return Err(e),
    //         },
    //         node.span(),
    //     )),
    //     _ => Err(StandardError::new("", node.span(), None)),
    // }

    Ok(IrNode::Break(IrBreakNode::new(IrSpan::from_span(
        node.span(),
    ))))
}
