use crate::lexing::position::Position;
use std::fmt::Display;

pub trait CommonNode: Display {
    fn position_start(&self) -> Option<Position>;
    fn position_end(&self) -> Option<Position>;
    fn clone_box(&self) -> Box<dyn CommonNode>;
}

impl Clone for Box<dyn CommonNode> {
    fn clone(&self) -> Box<dyn CommonNode> {
        self.clone_box()
    }
}
