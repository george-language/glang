use crate::symbol_table::SymbolTable;
use glang_attributes::Span;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Context {
    pub display_name: String,
    pub parent: Option<Rc<RefCell<Context>>>,
    pub parent_entry_span: Option<Span>,
    pub symbol_table: Option<Rc<RefCell<SymbolTable>>>,
}

impl Context {
    pub fn new(
        display_name: String,
        parent: Option<Rc<RefCell<Context>>>,
        parent_entry_span: Option<Span>,
        symbol_table: Option<Rc<RefCell<SymbolTable>>>,
    ) -> Self {
        Self {
            display_name,
            parent,
            parent_entry_span,
            symbol_table,
        }
    }
}
