use crate::lexing::position::Position;
use std::collections::HashMap;

pub struct Context {
    pub display_name: &'static str,
    pub parent: Option<Box<Context>>,
    pub parent_entry_pos: Option<Position>,
    pub symbol_table: Option<HashMap<&'static str, Option<String>>>,
}
