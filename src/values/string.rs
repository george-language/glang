use crate::{
    errors::standard_error::StandardError,
    interpreting::context::Context,
    lexing::position::Position,
    values::{number::Number, value::Value},
};

#[derive(Debug, Clone)]
pub struct Str {
    pub value: String,
    pub context: Option<Context>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl Str {
    pub fn new(value: String) -> Self {
        Str {
            value: value,
            context: None,
            pos_start: None,
            pos_end: None,
        }
    }

    pub fn from(string: &str) -> Box<Value> {
        Box::new(Value::StringValue(Str::new(string.to_string())))
    }

    pub fn perform_operation(
        &mut self,
        operator: &str,
        other: Box<Value>,
    ) -> (Option<Box<Value>>, Option<StandardError>) {
        match other.as_ref() {
            Value::StringValue(value) => match operator {
                "+" => {
                    let mut copy = self.clone();
                    copy.value.push_str(&value.value);

                    return (Some(Box::new(Value::StringValue(copy))), None);
                }
                "-" => {
                    let mut copy = self.clone();
                    copy.value.clear();
                    copy.value.push_str(&value.value);

                    return (Some(Box::new(Value::StringValue(copy))), None);
                }
                "==" => {
                    return (
                        Some(
                            Value::NumberValue(Number::new(
                                (self.value == value.value) as u8 as f64,
                            ))
                            .set_context(self.context.clone()),
                        ),
                        None,
                    );
                }
                "!=" => {
                    return (
                        Some(
                            Value::NumberValue(Number::new(
                                (self.value != value.value) as u8 as f64,
                            ))
                            .set_context(self.context.clone()),
                        ),
                        None,
                    );
                }
                "and" => {
                    return (
                        Some(
                            Value::NumberValue(Number::new(
                                (!self.value.is_empty() && !value.value.is_empty()) as u8 as f64,
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
                                (!self.value.is_empty() || !value.value.is_empty()) as u8 as f64,
                            ))
                            .set_context(self.context.clone()),
                        ),
                        None,
                    );
                }
                _ => (None, Some(self.illegal_operation(Some(other)))),
            },
            Value::NumberValue(value) => match operator {
                "*" => {
                    if value.value < 0.0 {
                        return (
                            None,
                            Some(StandardError::new(
                                "cannot multiply string by a negative value",
                                other.position_start().unwrap(),
                                other.position_end().unwrap(),
                                None,
                            )),
                        );
                    }

                    let mut copy = self.clone();
                    copy.value = self.value.repeat(value.value as usize);

                    return (Some(Box::new(Value::StringValue(copy))), None);
                }
                "^" => {
                    if value.value < -1.0 {
                        return (
                            None,
                            Some(StandardError::new(
                                "cannot access a negative index",
                                value.pos_start.clone().unwrap(),
                                value.pos_end.clone().unwrap(),
                                Some(
                                    "use an index greater than or equal to 0 or use -1 to reverse the string",
                                ),
                            )),
                        );
                    }

                    if value.value == -1.0 {
                        return (
                            Some(Str::from(
                                self.value.chars().rev().collect::<String>().as_str(),
                            )),
                            None,
                        );
                    }

                    if (value.value as usize) >= self.value.len() {
                        return (
                            None,
                            Some(StandardError::new(
                                "index is out of bounds",
                                value.pos_start.clone().unwrap(),
                                value.pos_end.clone().unwrap(),
                                None,
                            )),
                        );
                    }

                    return (
                        Some(Str::from(
                            self.value
                                .chars()
                                .nth(value.value as usize)
                                .unwrap()
                                .to_string()
                                .as_str(),
                        )),
                        None,
                    );
                }
                _ => (None, Some(self.illegal_operation(Some(other)))),
            },
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
