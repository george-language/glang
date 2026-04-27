use crate::{context::Context, values::value::Value};
use glang_attributes::{Span, StandardError};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Number {
    pub value: f64,
    pub context: Option<Rc<RefCell<Context>>>,
    pub is_const: bool,
    pub span: Span,
}

impl Number {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            context: None,
            is_const: false,
            span: Span::empty(),
        }
    }

    pub fn from(value: f64) -> Rc<RefCell<Value>> {
        Rc::new(RefCell::new(Value::NumberValue(Number::new(value))))
    }

    pub fn null_value() -> Rc<RefCell<Value>> {
        Rc::new(RefCell::new(Value::NumberValue(Number::new(0.0))))
    }

    pub fn true_value() -> Rc<RefCell<Value>> {
        Rc::new(RefCell::new(Value::NumberValue(Number::new(1.0))))
    }

    pub fn false_value() -> Rc<RefCell<Value>> {
        Rc::new(RefCell::new(Value::NumberValue(Number::new(0.0))))
    }

    pub fn perform_operation(
        &self,
        operator: &str,
        other: Rc<RefCell<Value>>,
    ) -> Result<Rc<RefCell<Value>>, StandardError> {
        match *other.borrow() {
            Value::NumberValue(ref value) => {
                let left_val = self.value;
                let right_val = value.value;

                let result = match operator {
                    "+" => Some(left_val + right_val),
                    "-" => Some(left_val - right_val),
                    "*" => Some(left_val * right_val),
                    "/" => {
                        if right_val == 0.0 {
                            return Err(StandardError::new(
                                "division by zero",
                                value.span.clone(),
                                None,
                            ));
                        }
                        Some(left_val / right_val)
                    }
                    "^" => {
                        if right_val <= 0.0 {
                            return Err(StandardError::new(
                                "powered by operator less than or equal to 0",
                                value.span.clone(),
                                None,
                            ));
                        }

                        Some(left_val.powf(right_val))
                    }
                    "%" => {
                        if right_val <= 0.0 {
                            return Err(StandardError::new(
                                "modded by operator less than or equal to 0",
                                value.span.clone(),
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
                    _ => return Err(self.illegal_operation(Some(other.clone()))),
                };

                let comparison_result = Number::from(result.unwrap());
                comparison_result
                    .borrow_mut()
                    .set_context(self.context.clone());

                Ok(comparison_result)
            }
            _ => Err(self.illegal_operation(Some(other.clone()))),
        }
    }

    pub fn illegal_operation(&self, other: Option<Rc<RefCell<Value>>>) -> StandardError {
        let (pos_end, help_msg) = if let Some(illegal) = other {
            (
                illegal.borrow().position_end().clone(),
                Some(format!(
                    "the left type is a number and the right type is a {}",
                    illegal.borrow().object_type()
                )),
            )
        } else {
            (self.span.end.clone(), None)
        };

        StandardError::new(
            "operation not supported by type",
            Span::new(&self.span.filename, self.span.start.clone(), pos_end),
            help_msg.as_deref(),
        )
    }
}
