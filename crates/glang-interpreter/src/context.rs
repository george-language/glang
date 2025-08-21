use crate::symbol_table::SymbolTable;
use glang_attributes::Position;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Context {
    pub display_name: String,
    pub parent: Option<Rc<RefCell<Context>>>,
    pub parent_entry_pos: Option<Position>,
    pub symbol_table: Option<Rc<RefCell<SymbolTable>>>,
}

impl Context {
    pub fn new(
        display_name: String,
        parent: Option<Rc<RefCell<Context>>>,
        parent_entry_pos: Option<Position>,
    ) -> Self {
        Self {
            display_name,
            parent,
            parent_entry_pos,
            symbol_table: None,
        }
    }
}
