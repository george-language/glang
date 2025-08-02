use crate::lexing::position::Position;

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
