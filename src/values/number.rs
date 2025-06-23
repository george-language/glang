use crate::{
    errors::standard_error::StandardError, interpreting::context::Context,
    lexing::position::Position, values::value::Value,
};

#[derive(Debug, Clone)]
pub struct Number {
    pub value: f64,
    pub context: Option<Context>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl Number {
    pub fn new(value: f64) -> Self {
        Number {
            value: value,
            context: None,
            pos_start: None,
            pos_end: None,
        }
    }

    pub fn from(value: f64) -> Box<Value> {
        Box::new(Value::NumberValue(Number::new(value)))
    }

    pub fn null_value() -> Box<Value> {
        Box::new(Value::NumberValue(Number::new(0.0)))
    }

    pub fn true_value() -> Box<Value> {
        Box::new(Value::NumberValue(Number::new(1.0)))
    }

    pub fn false_value() -> Box<Value> {
        Box::new(Value::NumberValue(Number::new(0.0)))
    }

    pub fn perform_operation(
        &self,
        operator: &'static str,
        other: Box<Value>,
    ) -> (Option<Box<Value>>, Option<StandardError>) {
        match other.as_ref() {
            Value::NumberValue(right) => {
                let left_val = self.value;
                let right_val = right.value;

                let result = match operator {
                    "+" => Some(left_val + right_val),
                    "-" => Some(left_val - right_val),
                    "*" => Some(left_val * right_val),
                    "/" => {
                        if right_val == 0.0 {
                            return (
                                None,
                                Some(StandardError::new(
                                    "division by zero",
                                    right.pos_start.clone().unwrap(),
                                    right.pos_end.clone().unwrap(),
                                    None,
                                )),
                            );
                        }
                        Some(left_val / right_val)
                    }
                    "^" => {
                        if right_val <= 0.0 {
                            return (
                                None,
                                Some(StandardError::new(
                                    "powered by operator less than or equal to 0",
                                    right.pos_start.clone().unwrap(),
                                    right.pos_end.clone().unwrap(),
                                    None,
                                )),
                            );
                        }
                        Some(left_val.powf(right_val))
                    }
                    "==" => Some((left_val == right_val) as u8 as f64),
                    "!=" => Some((left_val != right_val) as u8 as f64),
                    "<" => Some((left_val < right_val) as u8 as f64),
                    ">" => Some((left_val > right_val) as u8 as f64),
                    "<=" => Some((left_val <= right_val) as u8 as f64),
                    ">=" => Some((left_val >= right_val) as u8 as f64),
                    "and" => Some(((left_val != 0.0) && (right_val != 0.0)) as u8 as f64),
                    "or" => Some(((left_val != 0.0) || (right_val != 0.0)) as u8 as f64),
                    "oppositeof" => Some(if self.value == 0.0 { 1.0 } else { 0.0 }),
                    _ => return (None, Some(self.illegal_operation(Some(other)))),
                };

                (
                    Some(
                        Value::NumberValue(Number::new(result.unwrap()))
                            .set_context(self.context.clone()),
                    ),
                    None,
                )
            }
            _ => (None, Some(self.illegal_operation(Some(other)))),
        }
    }

    pub fn illegal_operation(&self, other: Option<Box<Value>>) -> StandardError {
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
        format!("{}", self.value).to_string()
    }
}
