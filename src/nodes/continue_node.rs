use crate::lexing::position::Position;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct ContinueNode {
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl ContinueNode {
    pub fn new(pos_start: Option<Position>, pos_end: Option<Position>) -> Self {
        Self {
            pos_start: pos_start,
            pos_end: pos_end,
        }
    }
}

impl Display for ContinueNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
