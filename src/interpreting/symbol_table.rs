use crate::values::value::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub symbols: HashMap<String, Option<Value>>,
    pub parent: Option<Box<SymbolTable>>,
}

impl SymbolTable {
    pub fn new(parent: Option<Box<SymbolTable>>) -> Self {
        Self {
            symbols: HashMap::new(),
            parent: parent,
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.symbols.get(name) {
            return value.clone();
        }

        if let Some(parent) = &self.parent {
            return parent.get(name);
        }

        None
    }

    pub fn set(&mut self, name: String, value: Option<Value>) {
        if name == "_" {
            return;
        }

        self.symbols.insert(name, value);
    }

    pub fn remove(&mut self, name: &str) {
        self.symbols.remove(name);
    }

    pub fn combined(
        &self,
        table: HashMap<String, Option<Value>>,
    ) -> HashMap<String, Option<Value>> {
        let mut new_map = self.symbols.clone();
        new_map.extend(table);

        new_map
    }
}
