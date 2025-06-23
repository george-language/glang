use crate::lexing::position::Position;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct BreakNode {
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl BreakNode {
    pub fn new(pos_start: Option<Position>, pos_end: Option<Position>) -> Self {
        BreakNode {
            pos_start: pos_start.clone(),
            pos_end: pos_end.clone(),
        }
    }
}

impl Display for BreakNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
