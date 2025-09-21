use crate::{
    context::Context,
    values::{number::Number, value::Value},
};
use glang_attributes::{Position, StandardError};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Str {
    pub value: String,
    pub context: Option<Rc<RefCell<Context>>>,
    pub is_const: bool,
    pub pos_start: Option<Rc<Position>>,
    pub pos_end: Option<Rc<Position>>,
}

impl Str {
    pub fn new(value: String) -> Self {
        Str {
            value,
            context: None,
            is_const: false,
            pos_start: None,
            pos_end: None,
        }
    }

    pub fn from(string: &str) -> Rc<RefCell<Value>> {
        Rc::new(RefCell::new(Value::StringValue(Str::new(
            string.to_string(),
        ))))
    }

    pub fn perform_operation(
        &self,
        operator: &str,
        other: Rc<RefCell<Value>>,
    ) -> Result<Rc<RefCell<Value>>, StandardError> {
        match *other.borrow_mut() {
            Value::StringValue(ref value) => match operator {
                "+" => {
                    let mut copy = self.clone();
                    copy.value.push_str(&value.value);

                    Ok(Rc::new(RefCell::new(Value::StringValue(copy))))
                }
                "-" => {
                    let mut copy = self.clone();
                    copy.value.clear();
                    copy.value.push_str(&value.value);

                    Ok(Rc::new(RefCell::new(Value::StringValue(copy))))
                }
                "==" => {
                    let is_neq = Number::from((self.value == value.value) as u8 as f64);
                    is_neq.borrow_mut().set_context(self.context.clone());

                    Ok(is_neq)
                }
                "!=" => {
                    let is_neq = Number::from((self.value != value.value) as u8 as f64);
                    is_neq.borrow_mut().set_context(self.context.clone());

                    Ok(is_neq)
                }
                ">" => {
                    let is_gt = Number::from((self.value > value.value) as u8 as f64);
                    is_gt.borrow_mut().set_context(self.context.clone());

                    Ok(is_gt)
                }
                "<" => {
                    let is_lt = Number::from((self.value < value.value) as u8 as f64);
                    is_lt.borrow_mut().set_context(self.context.clone());

                    Ok(is_lt)
                }
                ">=" => {
                    let is_gte = Number::from((self.value >= value.value) as u8 as f64);
                    is_gte.borrow_mut().set_context(self.context.clone());

                    Ok(is_gte)
                }
                "<=" => {
                    let is_lte = Number::from((self.value <= value.value) as u8 as f64);
                    is_lte.borrow_mut().set_context(self.context.clone());

                    Ok(is_lte)
                }
                "and" => {
                    let is_and = Number::from(
                        (!self.value.is_empty() && !value.value.is_empty()) as u8 as f64,
                    );
                    is_and.borrow_mut().set_context(self.context.clone());

                    Ok(is_and)
                }
                "or" => {
                    let is_or = Number::from(
                        (!self.value.is_empty() || !value.value.is_empty()) as u8 as f64,
                    );
                    is_or.borrow_mut().set_context(self.context.clone());

                    Ok(is_or)
                }
                _ => Err(self.illegal_operation(Some(other.clone()))),
            },
            Value::NumberValue(ref value) => match operator {
                "*" => {
                    if value.value < 0.0 {
                        return Err(StandardError::new(
                            "cannot multiply string by a negative value",
                            other.borrow().position_start().unwrap(),
                            other.borrow().position_end().unwrap(),
                            None,
                        ));
                    }

                    let mut copy = self.clone();
                    copy.value = self.value.repeat(value.value as usize);

                    Ok(Rc::new(RefCell::new(Value::StringValue(copy))))
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
                _ => Err(self.illegal_operation(Some(other.clone()))),
            },
            _ => Err(self.illegal_operation(Some(other.clone()))),
        }
    }

    pub fn illegal_operation(&self, other: Option<Rc<RefCell<Value>>>) -> StandardError {
        StandardError::new(
            "operation not supported by the string type",
            self.pos_start.as_ref().unwrap().clone(),
            if other.is_some() {
                other.unwrap().borrow().position_end().unwrap()
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
