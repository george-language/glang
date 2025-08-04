use std::{cell::RefCell, rc::Rc};

use crate::{
    errors::standard_error::StandardError, interpreting::context::Context,
    lexing::position::Position, values::value::Value,
};

#[derive(Debug, Clone)]
pub struct Number {
    pub value: f64,
    pub context: Option<Rc<RefCell<Context>>>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl Number {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            context: None,
            pos_start: None,
            pos_end: None,
        }
    }

    pub fn from(value: f64) -> Value {
        Value::NumberValue(Number::new(value))
    }

    pub fn null_value() -> Value {
        Value::NumberValue(Number::new(0.0))
    }

    pub fn true_value() -> Value {
        Value::NumberValue(Number::new(1.0))
    }

    pub fn false_value() -> Value {
        Value::NumberValue(Number::new(0.0))
    }

    pub fn perform_operation(&self, operator: &str, other: Value) -> Result<Value, StandardError> {
        match other {
            Value::NumberValue(ref right) => {
                let left_val = self.value;
                let right_val = right.value;

                let result = match operator {
                    "+" => Some(left_val + right_val),
                    "-" => Some(left_val - right_val),
                    "*" => Some(left_val * right_val),
                    "/" => {
                        if right_val == 0.0 {
                            return Err(StandardError::new(
                                "division by zero",
                                right.pos_start.clone().unwrap(),
                                right.pos_end.clone().unwrap(),
                                None,
                            ));
                        }
                        Some(left_val / right_val)
                    }
                    "^" => {
                        if right_val <= 0.0 {
                            return Err(StandardError::new(
                                "powered by operator less than or equal to 0",
                                right.pos_start.clone().unwrap(),
                                right.pos_end.clone().unwrap(),
                                None,
                            ));
                        }

                        Some(left_val.powf(right_val))
                    }
                    "%" => {
                        if right_val <= 0.0 {
                            return Err(StandardError::new(
                                "modded by operator less than or equal to 0",
                                right.pos_start.clone().unwrap(),
                                right.pos_end.clone().unwrap(),
                                None,
                            ));
                        }

                        Some(left_val.rem_euclid(right_val))
                    }
                    "==" => Some((left_val == right_val) as u8 as f64),
                    "!=" => Some((left_val != right_val) as u8 as f64),
                    "<" => Some((left_val < right_val) as u8 as f64),
                    ">" => Some((left_val > right_val) as u8 as f64),
                    "<=" => Some((left_val <= right_val) as u8 as f64),
                    ">=" => Some((left_val >= right_val) as u8 as f64),
                    "and" => Some(((left_val != 0.0) && (right_val != 0.0)) as u8 as f64),
                    "or" => Some(((left_val != 0.0) || (right_val != 0.0)) as u8 as f64),
                    "not" => Some(if self.value == 0.0 { 1.0 } else { 0.0 }),
                    _ => return Err(self.illegal_operation(Some(other))),
                };

                Ok(Value::NumberValue(Number::new(result.unwrap()))
                    .set_context(self.context.clone()))
            }
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

    pub fn as_string(&self) -> String {
        self.value.to_string()
    }
}
