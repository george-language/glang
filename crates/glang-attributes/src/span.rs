use crate::Position;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Span {
    pub filename: PathBuf,
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(filename: &Path, start: Position, end: Position) -> Self {
        Self {
            filename: filename.to_owned(),
            start,
            end,
        }
    }

    pub fn empty() -> Self {
        Self {
            filename: PathBuf::new(),
            start: Position::new(0, 0, 0),
            end: Position::new(0, 0, 0),
        }
    }
}
