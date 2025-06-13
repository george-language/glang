use std::collections::HashMap;

#[derive(Clone)]
pub struct SymbolTable {
    pub symbols: HashMap<&'static str, Option<String>>,
    pub parent: Option<Box<SymbolTable>>,
}

impl SymbolTable {
    pub fn new(parent: Option<Box<SymbolTable>>) -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            parent: parent,
        }
    }

    pub fn get(&self, name: &'static str) -> Option<&str> {
        let value = self.symbols.get(&name).clone();

        if value.is_none() && self.parent.is_some() {
            return self.parent.as_ref().unwrap().get(&name).clone();
        }

        value.unwrap().as_deref()
    }

    pub fn set(&mut self, name: &'static str, value: Option<String>) {
        self.symbols.insert(name, value);
    }

    pub fn remove(&mut self, name: &'static str) {
        self.symbols.remove(name);
    }

    pub fn combined(
        &self,
        table: HashMap<&'static str, Option<String>>,
    ) -> HashMap<&'static str, Option<String>> {
        let mut new_map = self.symbols.clone();
        new_map.extend(table);

        new_map
    }
}
