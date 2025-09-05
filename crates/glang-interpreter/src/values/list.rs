use crate::{
    context::Context,
    values::{number::Number, value::Value},
};
use glang_attributes::{Position, StandardError};
use std::{cell::RefCell, iter::zip, rc::Rc};

#[derive(Debug, Clone)]
pub struct List {
    pub elements: Vec<Value>,
    pub context: Option<Rc<RefCell<Context>>>,
    pub is_const: bool,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl List {
    pub fn new(elements: Vec<Value>) -> Self {
        Self {
            elements,
            context: None,
            is_const: false,
            pos_start: None,
            pos_end: None,
        }
    }

    pub fn from(elements: Vec<Value>) -> Value {
        Value::ListValue(List::new(elements))
    }

    pub fn perform_operation(self, operator: &str, other: Value) -> Result<Value, StandardError> {
        if operator == "*" {
            return Ok(self.push(other.clone()));
        }

        match other {
            Value::ListValue(ref value) => match operator {
                "+" => Ok(self.append(&mut value.elements.clone())),
                "==" => {
                    let mut is_eq = Number::null_value();
                    is_eq.set_context(self.context.clone());

                    for (a, b) in zip(&self.elements, &value.elements) {
                        let result = a.to_owned().perform_operation("==", b.to_owned());

                        if result.is_err() {
                            return Err(result.err().unwrap());
                        }

                        is_eq = result.ok().unwrap();
                    }

                    Ok(is_eq)
                }
                "!=" => {
                    let mut is_neq = Number::null_value();
                    is_neq.set_context(self.context.clone());

                    for (a, b) in zip(&self.elements, &value.elements) {
                        let result = a.to_owned().perform_operation("!=", b.to_owned());

                        if result.is_err() {
                            return Err(result.err().unwrap());
                        }

                        is_neq = result.ok().unwrap();
                    }

                    Ok(is_neq)
                }
                "and" => {
                    let mut is_and = Number::from(
                        (!self.elements.is_empty() && !value.elements.is_empty()) as u8 as f64,
                    );
                    is_and.set_context(self.context.clone());

                    Ok(is_and)
                }
                "or" => {
                    let mut is_or = Number::from(
                        (!self.elements.is_empty() || !value.elements.is_empty()) as u8 as f64,
                    );
                    is_or.set_context(self.context.clone());

                    Ok(is_or)
                }
                _ => Err(self.illegal_operation(Some(other))),
            },
            Value::NumberValue(ref value) => match operator {
                "^" => {
                    if value.value < -1.0 {
                        return Err(StandardError::new(
                            "cannot access a negative index",
                            value.pos_start.clone().unwrap(),
                            value.pos_end.clone().unwrap(),
                            Some(
                                "use an index greater than or equal to 0 or use -1 to reverse the list",
                            ),
                        ));
                    }

                    if value.value == -1.0 {
                        return Ok(self.reverse());
                    }

                    if (value.value as usize) >= self.elements.len() {
                        return Err(StandardError::new(
                            "index is out of bounds",
                            value.pos_start.clone().unwrap(),
                            value.pos_end.clone().unwrap(),
                            None,
                        ));
                    }

                    Ok(self.retrieve(value.value as usize))
                }
                "-" => {
                    if value.value < 0.0 {
                        return Err(StandardError::new(
                            "cannot access a negative index",
                            value.pos_start.clone().unwrap(),
                            value.pos_end.clone().unwrap(),
                            Some("use an index greater than or equal to 0"),
                        ));
                    }

                    if (value.value as usize) >= self.elements.len() {
                        return Err(StandardError::new(
                            "index is out of bounds",
                            value.pos_start.clone().unwrap(),
                            value.pos_end.clone().unwrap(),
                            None,
                        ));
                    }

                    Ok(self.remove(value.value as usize))
                }
                _ => Err(self.illegal_operation(Some(other))),
            },
            _ => Err(self.illegal_operation(Some(other))),
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

    pub fn push(mut self, item: Value) -> Value {
        self.elements.push(item);

        Value::ListValue(self)
    }

    pub fn append(mut self, other: &mut Vec<Value>) -> Value {
        self.elements.append(other);

        Value::ListValue(self)
    }

    pub fn remove(mut self, index: usize) -> Value {
        self.elements.remove(index);

        Value::ListValue(self)
    }

    pub fn retrieve(&self, index: usize) -> Value {
        self.elements[index].clone()
    }

    pub fn reverse(mut self) -> Value {
        self.elements.reverse();

        Value::ListValue(self)
    }

    pub fn as_string(&self) -> String {
        let output = self
            .elements
            .iter()
            .map(|item| item.as_string())
            .collect::<Vec<_>>()
            .join(", ");

        format!("[{output}]").to_string()
    }
}
