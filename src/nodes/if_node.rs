use crate::{lexing::position::Position, nodes::common_node::CommonNode};
use std::{any::Any, fmt::Display};

#[derive(Clone)]
pub struct IfNode {
    pub cases: Vec<(Box<dyn CommonNode>, Box<dyn CommonNode>, bool)>,
    pub else_case: Option<(Box<dyn CommonNode>, bool)>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl IfNode {
    pub fn new(
        cases: Vec<(Box<dyn CommonNode>, Box<dyn CommonNode>, bool)>,
        else_case: Option<(Box<dyn CommonNode>, bool)>,
    ) -> Self {
        IfNode {
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

impl CommonNode for IfNode {
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

impl Display for IfNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
