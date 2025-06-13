use crate::{
    errors::standard_error::StandardError, interpreting::context::Context,
    lexing::position::Position, values::value::Value,
};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct List {
    pub elements: Vec<Option<Box<Value>>>,
    pub context: Option<Context>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl List {
    pub fn new(elements: Vec<Option<Box<Value>>>) -> Self {
        List {
            elements: elements,
            context: None,
            pos_start: None,
            pos_end: None,
        }
    }

    pub fn illegal_operation(&self, other: Option<Box<Value>>) -> StandardError {
        StandardError::new(
            "operation not supported by type".to_string(),
            self.pos_start.as_ref().unwrap().clone(),
            if other.is_some() {
                other.unwrap().position_end().unwrap()
            } else {
                self.pos_end.as_ref().unwrap().clone()
            },
            None,
        )
    }

    pub fn as_string(&self) -> String {
        let mut output = self
            .elements
            .iter()
            .map(|item| item.as_ref().unwrap().as_string())
            .collect::<Vec<_>>()
            .join(", ");

        if self.elements.is_empty() {
            output = "[]".to_string();
        }

        format!("{}", output).to_string()
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<list: {}>", self)
    }
}
