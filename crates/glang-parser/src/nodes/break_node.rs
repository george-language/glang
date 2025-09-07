use glang_attributes::Position;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct BreakNode {
    pub pos_start: Option<Rc<Position>>,
    pub pos_end: Option<Rc<Position>>,
}

impl BreakNode {
    pub fn new(pos_start: Option<Rc<Position>>, pos_end: Option<Rc<Position>>) -> Self {
        Self { pos_start, pos_end }
    }
}
