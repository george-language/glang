use crate::symbol_table::SymbolTable;
use glang_attributes::Span;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Context {
    pub parent: Option<Rc<RefCell<Context>>>,
    pub parent_entry_span: Option<Span>,
    pub symbol_table: Rc<RefCell<SymbolTable>>,
}

impl Context {
    pub fn new(
        parent: Option<Rc<RefCell<Context>>>,
        parent_entry_span: Option<Span>,
        symbol_table: Rc<RefCell<SymbolTable>>,
    ) -> Self {
        Self {
            parent,
            parent_entry_span,
            symbol_table,
        }
    }
}
