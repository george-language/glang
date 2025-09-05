use crate::nodes::ast_node::AstNode;
use glang_attributes::Position;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct IfNode {
    pub cases: Arc<[(Box<AstNode>, Box<AstNode>, bool)]>,
    pub else_case: Option<(Box<AstNode>, bool)>,
    pub pos_start: Option<Rc<Position>>,
    pub pos_end: Option<Rc<Position>>,
}

impl IfNode {
    pub fn new(
        cases: &[(Box<AstNode>, Box<AstNode>, bool)],
        else_case: Option<(Box<AstNode>, bool)>,
    ) -> Self {
        Self {
            cases: Arc::from(cases),
            else_case: else_case.to_owned(),
            pos_start: cases[0].0.position_start(),
            pos_end: if else_case.is_none() {
                cases[cases.len() - 1].0.position_start()
            } else {
                else_case.unwrap().0.position_end()
            },
        }
    }
}
