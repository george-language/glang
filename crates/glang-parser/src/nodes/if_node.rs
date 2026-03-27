use crate::nodes::ast_node::AstNode;
use glang_attributes::Span;
use std::{path::Path, rc::Rc};

#[derive(Debug, Clone)]
pub struct IfNode {
    pub cases: Rc<[(Box<AstNode>, Box<AstNode>, bool)]>,
    pub else_case: Option<(Box<AstNode>, bool)>,
    pub span: Span,
}

impl IfNode {
    pub fn new(
        cases: &[(Box<AstNode>, Box<AstNode>, bool)],
        else_case: Option<(Box<AstNode>, bool)>,
    ) -> Self {
        Self {
            cases: Rc::from(cases),
            else_case: else_case.to_owned(),
            span: Span::new(
                &cases[0].0.span().filename,
                cases[0].0.position_start(),
                if else_case.is_none() {
                    cases[cases.len() - 1].0.position_start()
                } else {
                    else_case.unwrap().0.position_end()
                },
            ),
        }
    }
}
