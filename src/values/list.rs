use crate::{
    errors::standard_error::StandardError,
    interpreting::context::Context,
    lexing::position::Position,
    values::{number::Number, value::Value},
};
use std::iter::zip;

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

    pub fn from(elements: Vec<Option<Box<Value>>>) -> Box<Value> {
        Box::new(Value::ListValue(List::new(elements)))
    }

    pub fn perform_operation(
        &mut self,
        operator: &'static str,
        other: Box<Value>,
    ) -> (Option<Box<Value>>, Option<StandardError>) {
        match other.as_ref() {
            Value::ListValue(right) => match operator {
                "+" => {
                    return (Some(self.append(&mut right.elements.clone())), None);
                }
                "*" => {
                    return (Some(self.push(Some(other.clone()))), None);
                }
                "==" => {
                    if self.elements.len() != right.elements.len() {
                        return (
                            Some(Number::false_value().set_context(self.context.clone())),
                            None,
                        );
                    } else {
                        for (a, b) in zip(self.elements.clone(), right.elements.clone()) {
                            let (result, error) = a.unwrap().perform_operation("==", b.unwrap());

                            if error.is_some() {
                                return (None, error);
                            }
                            if !result.is_some() {
                                return (
                                    Some(Number::false_value().set_context(self.context.clone())),
                                    None,
                                );
                            }
                        }
                    }

                    return (
                        Some(Number::true_value().set_context(self.context.clone())),
                        None,
                    );
                }
                "!=" => {
                    if self.elements.len() != right.elements.len() {
                        return (
                            Some(Number::false_value().set_context(self.context.clone())),
                            None,
                        );
                    } else {
                        for (a, b) in zip(self.elements.clone(), right.elements.clone()) {
                            let (result, error) = a.unwrap().perform_operation("==", b.unwrap());

                            if error.is_some() {
                                return (None, error);
                            }
                            if !result.is_some() {
                                return (
                                    Some(Number::true_value().set_context(self.context.clone())),
                                    None,
                                );
                            }
                        }
                    }

                    return (
                        Some(Number::false_value().set_context(self.context.clone())),
                        None,
                    );
                }
                "and" => {
                    return (
                        Some(
                            Value::NumberValue(Number::new(
                                (!self.elements.is_empty() && !right.elements.is_empty()) as u8
                                    as f64,
                            ))
                            .set_context(self.context.clone()),
                        ),
                        None,
                    );
                }
                "or" => {
                    return (
                        Some(
                            Value::NumberValue(Number::new(
                                (!self.elements.is_empty() || !right.elements.is_empty()) as u8
                                    as f64,
                            ))
                            .set_context(self.context.clone()),
                        ),
                        None,
                    );
                }
                _ => return (None, Some(self.illegal_operation(Some(other)))),
            },
            Value::NumberValue(right) => match operator {
                "*" => {
                    return (Some(self.push(Some(other.clone()))), None);
                }
                "^" => {
                    if right.value < -1.0 {
                        return (
                            None,
                            Some(StandardError::new(
                                "cannot access a negative index".to_string(),
                                right.pos_start.clone().unwrap(),
                                right.pos_end.clone().unwrap(),
                                Some("use an index greater than or equal to 0".to_string()),
                            )),
                        );
                    }

                    if right.value == -1.0 {
                        return (Some(self.reverse()), None);
                    }

                    if (right.value as usize) > self.elements.len() {
                        return (
                            None,
                            Some(StandardError::new(
                                "index is out of bounds".to_string(),
                                right.pos_start.clone().unwrap(),
                                right.pos_end.clone().unwrap(),
                                None,
                            )),
                        );
                    }

                    return (self.retrieve(right.value as usize), None);
                }
                "-" => {
                    if right.value < 0.0 {
                        return (
                            None,
                            Some(StandardError::new(
                                "cannot access a negative index".to_string(),
                                right.pos_start.clone().unwrap(),
                                right.pos_end.clone().unwrap(),
                                Some("use an index greater than or equal to 0".to_string()),
                            )),
                        );
                    }

                    if (right.value as usize) > self.elements.len() {
                        return (
                            None,
                            Some(StandardError::new(
                                "index is out of bounds".to_string(),
                                right.pos_start.clone().unwrap(),
                                right.pos_end.clone().unwrap(),
                                None,
                            )),
                        );
                    }

                    return (Some(self.remove(right.value as usize)), None);
                }
                _ => return (None, Some(self.illegal_operation(Some(other)))),
            },
            _ => {
                if operator == "*" {
                    return (Some(self.push(Some(other.clone()))), None);
                }

                return (None, Some(self.illegal_operation(Some(other))));
            }
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

    pub fn push(&mut self, item: Option<Box<Value>>) -> Box<Value> {
        let mut copy = self.clone();
        copy.elements.push(item);

        Box::new(Value::ListValue(copy))
    }

    pub fn append(&mut self, other: &mut Vec<Option<Box<Value>>>) -> Box<Value> {
        let mut copy = self.clone();
        copy.elements.append(other);

        Box::new(Value::ListValue(copy))
    }

    pub fn remove(&mut self, index: usize) -> Box<Value> {
        let mut copy = self.clone();
        copy.elements.remove(index).unwrap();

        Box::new(Value::ListValue(copy))
    }

    pub fn retrieve(&self, index: usize) -> Option<Box<Value>> {
        self.elements[index].clone()
    }

    pub fn reverse(&mut self) -> Box<Value> {
        let mut copy = self.clone();
        copy.elements.reverse();

        Box::new(Value::ListValue(copy))
    }

    pub fn as_string(&self) -> String {
        let output = self
            .elements
            .iter()
            .map(|item| {
                item.as_ref().map_or_else(
                    || Number::null_value().as_string(),
                    |boxed| boxed.as_string(),
                )
            })
            .collect::<Vec<_>>()
            .join(", ");

        format!("[{}]", output).to_string()
    }
}
