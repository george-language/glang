use std::iter::zip;

use crate::{
    errors::standard_error::StandardError,
    interpreting::context::Context,
    lexing::position::Position,
    values::{number::Number, value::Value},
};

#[derive(Debug, Clone)]
pub struct List {
    pub elements: Vec<Value>,
    pub context: Option<Context>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl List {
    pub fn new(elements: Vec<Value>) -> Self {
        Self {
            elements: elements,
            context: None,
            pos_start: None,
            pos_end: None,
        }
    }

    pub fn from(elements: Vec<Value>) -> Value {
        Value::ListValue(List::new(elements))
    }

    pub fn perform_operation(
        &mut self,
        operator: &str,
        other: Value,
    ) -> Result<Value, StandardError> {
        match other {
            Value::ListValue(ref right) => match operator {
                "+" => {
                    return Ok(self.append(&mut right.elements.clone()));
                }
                "*" => {
                    return Ok(self.push(other.clone()));
                }
                "==" => {
                    let mut is_eq = Number::null_value();

                    for (a, b) in zip(&self.elements, &right.elements) {
                        let result = a.to_owned().perform_operation("==", b.to_owned());

                        if result.is_err() {
                            return Err(result.err().unwrap());
                        }

                        is_eq = result.ok().unwrap();
                    }

                    return Ok(is_eq.set_context(self.context.clone()));
                }
                "!=" => {
                    let mut is_neq = Number::null_value();

                    for (a, b) in zip(&self.elements, &right.elements) {
                        let result = a.to_owned().perform_operation("!=", b.to_owned());

                        if result.is_err() {
                            return Err(result.err().unwrap());
                        }

                        is_neq = result.ok().unwrap();
                    }

                    return Ok(is_neq.set_context(self.context.clone()));
                }
                "and" => {
                    return Ok(Value::NumberValue(Number::new(
                        (!self.elements.is_empty() && !right.elements.is_empty()) as u8 as f64,
                    ))
                    .set_context(self.context.clone()));
                }
                "or" => {
                    return Ok(Value::NumberValue(Number::new(
                        (!self.elements.is_empty() || !right.elements.is_empty()) as u8 as f64,
                    ))
                    .set_context(self.context.clone()));
                }
                _ => return Err(self.illegal_operation(Some(other))),
            },
            Value::NumberValue(ref right) => match operator {
                "*" => {
                    return Ok(self.push(other.clone()));
                }
                "^" => {
                    if right.value < -1.0 {
                        return Err(StandardError::new(
                            "cannot access a negative index",
                            right.pos_start.clone().unwrap(),
                            right.pos_end.clone().unwrap(),
                            Some(
                                "use an index greater than or equal to 0 or use -1 to reverse the list",
                            ),
                        ));
                    }

                    if right.value == -1.0 {
                        return Ok(self.reverse());
                    }

                    if (right.value as usize) >= self.elements.len() {
                        return Err(StandardError::new(
                            "index is out of bounds",
                            right.pos_start.clone().unwrap(),
                            right.pos_end.clone().unwrap(),
                            None,
                        ));
                    }

                    return Ok(self.retrieve(right.value as usize));
                }
                "-" => {
                    if right.value < 0.0 {
                        return Err(StandardError::new(
                            "cannot access a negative index",
                            right.pos_start.clone().unwrap(),
                            right.pos_end.clone().unwrap(),
                            Some("use an index greater than or equal to 0"),
                        ));
                    }

                    if (right.value as usize) >= self.elements.len() {
                        return Err(StandardError::new(
                            "index is out of bounds",
                            right.pos_start.clone().unwrap(),
                            right.pos_end.clone().unwrap(),
                            None,
                        ));
                    }

                    return Ok(self.remove(right.value as usize));
                }
                _ => return Err(self.illegal_operation(Some(other))),
            },
            _ => {
                if operator == "*" {
                    return Ok(self.push(other.clone()));
                }

                return Err(self.illegal_operation(Some(other)));
            }
        }
    }

    pub fn illegal_operation(&self, other: Option<Value>) -> StandardError {
        StandardError::new(
            "operation not supported by type",
            self.pos_start.as_ref().unwrap().clone(),
            if other.is_some() {
                other.unwrap().position_end().unwrap()
            } else {
                self.pos_end.as_ref().unwrap().clone()
            },
            None,
        )
    }

    pub fn push(&mut self, item: Value) -> Value {
        let mut copy = self.clone();
        copy.elements.push(item);

        Value::ListValue(copy)
    }

    pub fn append(&mut self, other: &mut Vec<Value>) -> Value {
        let mut copy = self.clone();
        copy.elements.append(other);

        Value::ListValue(copy)
    }

    pub fn remove(&mut self, index: usize) -> Value {
        let mut copy = self.clone();
        copy.elements.remove(index);

        Value::ListValue(copy)
    }

    pub fn retrieve(&self, index: usize) -> Value {
        self.elements[index].clone()
    }

    pub fn reverse(&mut self) -> Value {
        let mut copy = self.clone();
        copy.elements.reverse();

        Value::ListValue(copy)
    }

    pub fn as_string(&self) -> String {
        let output = self
            .elements
            .iter()
            .map(|item| item.as_string())
            .collect::<Vec<_>>()
            .join(", ");

        format!("[{}]", output).to_string()
    }
}
