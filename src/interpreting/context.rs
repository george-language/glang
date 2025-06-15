use crate::{interpreting::symbol_table::SymbolTable, lexing::position::Position};

#[derive(Debug, Clone)]
pub struct Context {
    pub display_name: String,
    pub parent: Option<Box<Context>>,
    pub parent_entry_pos: Option<Position>,
    pub symbol_table: Option<SymbolTable>,
}

impl Context {
    pub fn new(
        display_name: String,
        parent: Option<Box<Context>>,
        parent_entry_pos: Option<Position>,
    ) -> Self {
        Context {
            display_name: display_name,
            parent: parent,
            parent_entry_pos: parent_entry_pos,
            symbol_table: None,
        }
    }
}
