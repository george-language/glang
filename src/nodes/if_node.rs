use crate::{
    lexing::{position::Position, token::Token},
    nodes::common_node::CommonNode,
    parsing::parse_result::ParseResult,
};
use std::fmt::Display;

#[derive(Clone)]
pub struct IfNode {
    pub cases: Vec<ParseResult>,
    pub else_case: ParseResult,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl IfNode {
    // pub fn new(cases: Vec<ParseResult>, else_case: ParseResult) -> Self {
    //     self.pos_start = self.cases[0][0].pos_start
    //     self.pos_end = (self.else_case or self.cases[len(self.cases) - 1])[0].pos_end

    //     IfNode {
    //         cases: cases,
    //         else_case: else_case,
    //         pos_start: pos_start,
    //         pos_end: pos_end,
    //     }
    // }
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
