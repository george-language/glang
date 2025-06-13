use std::fmt::Display;

use crate::{
    errors::standard_error::StandardError, interpreting::context::Context,
    lexing::position::Position, values::number::Number,
};

#[derive(Clone)]
pub enum Value {
    NumberValue(Number),
}

impl Value {
    pub fn position_start(&self) -> Option<Position> {
        match self {
            Value::NumberValue(number) => number.pos_start.clone(),
        }
    }

    pub fn position_end(&self) -> Option<Position> {
        match self {
            Value::NumberValue(number) => number.pos_end.clone(),
        }
    }

    pub fn set_position(
        &mut self,
        pos_start: Option<Position>,
        pos_end: Option<Position>,
    ) -> Box<Value> {
        match self {
            Value::NumberValue(number) => {
                number.pos_start = pos_start;
                number.pos_end = pos_end;
            }
        }

        Box::new(self.clone())
    }

    pub fn set_context(&mut self, context: Option<Context>) -> Box<Value> {
        match self {
            Value::NumberValue(number) => number.context = context,
        }

        Box::new(self.clone())
    }

    pub fn added_to(&self, other: Box<Value>) -> (Option<Box<Value>>, Option<StandardError>) {
        match self {
            Value::NumberValue(number) => number.added_to(other),
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Value::NumberValue(number) => number.as_string(),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
