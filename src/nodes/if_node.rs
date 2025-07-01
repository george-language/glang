use crate::{lexing::position::Position, nodes::ast_node::AstNode};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct IfNode {
    pub cases: Vec<(Box<AstNode>, Box<AstNode>, bool)>,
    pub else_case: Option<(Box<AstNode>, bool)>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl IfNode {
    pub fn new(
        cases: Vec<(Box<AstNode>, Box<AstNode>, bool)>,
        else_case: Option<(Box<AstNode>, bool)>,
    ) -> Self {
        Self {
            cases: cases.clone(),
            else_case: else_case.clone(),
            pos_start: Some(cases[0].0.position_start().unwrap()),
            pos_end: if else_case.is_none() {
                Some(cases[cases.len() - 1].0.position_start().unwrap().clone())
            } else {
                Some(else_case.unwrap().0.position_end().unwrap())
            },
        }
    }
}

impl Display for IfNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
