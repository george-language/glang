use std::{cell::RefCell, rc::Rc};

use crate::{
    errors::standard_error::StandardError,
    interpreting::context::Context,
    lexing::position::Position,
    values::{number::Number, value::Value},
};

#[derive(Debug, Clone)]
pub struct Str {
    pub value: String,
    pub context: Option<Rc<RefCell<Context>>>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl Str {
    pub fn new(value: String) -> Self {
        Str {
            value,
            context: None,
            pos_start: None,
            pos_end: None,
        }
    }

    pub fn from(string: &str) -> Value {
        Value::StringValue(Str::new(string.to_string()))
    }

    pub fn perform_operation(
        &mut self,
        operator: &str,
        other: Value,
    ) -> Result<Value, StandardError> {
        match other {
            Value::StringValue(ref value) => match operator {
                "+" => {
                    let mut copy = self.clone();
                    copy.value.push_str(&value.value);

                    Ok(Value::StringValue(copy))
                }
                "-" => {
                    let mut copy = self.clone();
                    copy.value.clear();
                    copy.value.push_str(&value.value);

                    Ok(Value::StringValue(copy))
                }
                "==" => {
                    Ok(Value::NumberValue(Number::new(
                        (self.value == value.value) as u8 as f64,
                    ))
                    .set_context(self.context.clone()))
                }
                "!=" => {
                    Ok(Value::NumberValue(Number::new(
                        (self.value != value.value) as u8 as f64,
                    ))
                    .set_context(self.context.clone()))
                }
                "and" => {
                    Ok(Value::NumberValue(Number::new(
                        (!self.value.is_empty() && !value.value.is_empty()) as u8 as f64,
                    ))
                    .set_context(self.context.clone()))
                }
                "or" => {
                    Ok(Value::NumberValue(Number::new(
                        (!self.value.is_empty() || !value.value.is_empty()) as u8 as f64,
                    ))
                    .set_context(self.context.clone()))
                }
                _ => Err(self.illegal_operation(Some(&other))),
            },
            Value::NumberValue(ref value) => match operator {
                "*" => {
                    if value.value < 0.0 {
                        return Err(StandardError::new(
                            "cannot multiply string by a negative value",
                            other.position_start().unwrap(),
                            other.position_end().unwrap(),
                            None,
                        ));
                    }

                    let mut copy = self.clone();
                    copy.value = self.value.repeat(value.value as usize);

                    Ok(Value::StringValue(copy))
                }
                "^" => {
                    if value.value < -1.0 {
                        return Err(StandardError::new(
                            "cannot access a negative index",
                            value.pos_start.clone().unwrap(),
                            value.pos_end.clone().unwrap(),
                            Some(
                                "use an index greater than or equal to 0 or use -1 to reverse the string",
                            ),
                        ));
                    }

                    if value.value == -1.0 {
                        return Ok(Str::from(
                            self.value.chars().rev().collect::<String>().as_str(),
                        ));
                    }

                    if (value.value as usize) >= self.value.len() {
                        return Err(StandardError::new(
                            "index is out of bounds",
                            value.pos_start.clone().unwrap(),
                            value.pos_end.clone().unwrap(),
                            None,
                        ));
                    }

                    Ok(Str::from(
                        self.value
                            .chars()
                            .nth(value.value as usize)
                            .unwrap()
                            .to_string()
                            .as_str(),
                    ))
                }
                _ => Err(self.illegal_operation(Some(&other))),
            },
            _ => Err(self.illegal_operation(Some(&other))),
        }
    }

    pub fn illegal_operation(&self, other: Option<&Value>) -> StandardError {
        StandardError::new(
            "operation not supported by the string type",
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
        self.value.clone()
    }
}
