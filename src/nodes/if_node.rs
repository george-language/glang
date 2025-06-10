use crate::{
    lexing::{position::Position, token::Token},
    nodes::common_node::CommonNode,
    parsing::parse_result::ParseResult,
};
use std::fmt::Display;

#[derive(Clone)]
pub struct IfNode {
    pub cases: Vec<ParseResult>,
    pub else_case: Option<ParseResult>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl IfNode {
    pub fn new(cases: Vec<ParseResult>, else_case: Option<ParseResult>) -> Self {
        IfNode {
            cases: cases.clone(),
            else_case: else_case.clone(),
            pos_start: Some(
                cases[0]
                    .node
                    .as_ref()
                    .unwrap()
                    .position_start()
                    .unwrap()
                    .clone(),
            ),
            pos_end: if else_case.is_none() {
                Some(
                    cases[cases.len() - 1]
                        .node
                        .as_ref()
                        .unwrap()
                        .position_start()
                        .unwrap()
                        .clone(),
                )
            } else {
                Some(else_case.unwrap().node.unwrap().position_end().unwrap())
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
}

impl Display for IfNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
