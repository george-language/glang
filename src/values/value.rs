use std::fmt::Display;

use crate::{
    errors::standard_error::StandardError,
    interpreting::{context::Context, runtime_result::RuntimeResult},
    lexing::position::Position,
    values::{list::List, number::Number},
};

#[derive(Debug, Clone)]
pub enum Value {
    NumberValue(Number),
    ListValue(List),
}

impl Value {
    pub fn position_start(&self) -> Option<Position> {
        match self {
            Value::NumberValue(value) => value.pos_start.clone(),
            Value::ListValue(value) => value.pos_start.clone(),
        }
    }

    pub fn position_end(&self) -> Option<Position> {
        match self {
            Value::NumberValue(value) => value.pos_end.clone(),
            Value::ListValue(value) => value.pos_end.clone(),
        }
    }

    pub fn set_position(
        &mut self,
        pos_start: Option<Position>,
        pos_end: Option<Position>,
    ) -> Box<Value> {
        match self {
            Value::NumberValue(value) => {
                value.pos_start = pos_start;
                value.pos_end = pos_end;
            }
            Value::ListValue(value) => {
                value.pos_start = pos_start;
                value.pos_end = pos_end;
            }
        }

        Box::new(self.clone())
    }

    pub fn set_context(&mut self, context: Option<Context>) -> Box<Value> {
        match self {
            Value::NumberValue(value) => value.context = context,
            Value::ListValue(value) => value.context = context,
        }

        Box::new(self.clone())
    }

    pub fn perform_operation(
        &mut self,
        operator: &'static str,
        other: Box<Value>,
    ) -> (Option<Box<Value>>, Option<StandardError>) {
        match self {
            Value::NumberValue(value) => value.perform_operation(operator, other),
            Value::ListValue(value) => value.perform_operation(operator, other),
            _ => (
                None,
                Some(StandardError::new(
                    format!("type doesn't support the '{}' operator", operator).to_string(),
                    self.position_start().unwrap(),
                    self.position_end().unwrap(),
                    None,
                )),
            ),
        }
    }

    pub fn object_type(&self) -> &'static str {
        match self {
            Value::NumberValue(_) => "number",
            Value::ListValue(_) => "list",
            _ => "null",
        }
    }

    pub fn is_true(&self) -> bool {
        match self {
            Value::NumberValue(value) => value.value == 1,
            Value::ListValue(value) => value.elements.is_empty(),
            _ => false,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Value::NumberValue(value) => value.as_string(),
            Value::ListValue(value) => value.as_string(),
            _ => "".to_string(),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
