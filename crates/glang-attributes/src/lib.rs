extern crate simply_colored;

mod keywords;
mod position;
mod span;
mod standard_error;

pub use keywords::{BUILT_IN_FUNCTIONS, DIGITS, KEYWORDS, LETTERS, LETTERS_DIGITS};
pub use position::Position;
pub use span::Span;
pub use standard_error::StandardError;
