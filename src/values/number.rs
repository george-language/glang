use crate::{
    errors::standard_error::StandardError, interpreting::context::Context,
    lexing::position::Position, values::value::Value,
};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Number {
    pub value: Option<isize>,
    pub context: Option<Context>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl Number {
    pub fn new(value: Option<isize>) -> Self {
        Number {
            value: value,
            context: None,
            pos_start: None,
            pos_end: None,
        }
    }

    pub fn perform_operation(
        &self,
        operator: &'static str,
        other: Box<Value>,
    ) -> (Option<Box<Value>>, Option<StandardError>) {
        match other.as_ref() {
            Value::NumberValue(right) => {
                let left_val = self.value.unwrap();
                let right_val = right.value.unwrap();

                let result = match operator {
                    "+" => Some(left_val + right_val),
                    "-" => Some(left_val - right_val),
                    "*" => Some(left_val * right_val),
                    "/" => {
                        if right_val == 0 {
                            return (
                                None,
                                Some(StandardError::new(
                                    "division by zero".to_string(),
                                    right.pos_start.clone().unwrap(),
                                    right.pos_end.clone().unwrap(),
                                    None,
                                )),
                            );
                        }

                        Some(left_val / right_val)
                    }
                    "^" => {
                        if right_val <= 0 {
                            return (
                                None,
                                Some(StandardError::new(
                                    "powered by operator less than or equal to 0".to_string(),
                                    right.pos_start.clone().unwrap(),
                                    right.pos_end.clone().unwrap(),
                                    None,
                                )),
                            );
                        }

                        Some(left_val.pow(right_val as u32) as isize)
                    }
                    "==" => Some((left_val == right_val) as isize),
                    "!=" => Some((left_val != right_val) as isize),
                    "<" => Some((left_val < right_val) as isize),
                    ">" => Some((left_val > right_val) as isize),
                    "<=" => Some((left_val <= right_val) as isize),
                    ">=" => Some((left_val >= right_val) as isize),
                    "and" => Some(((left_val != 0) && (right_val != 0)) as isize),
                    "or" => Some(((left_val != 0) || (right_val != 0)) as isize),
                    "oppositeof" => Some(if self.value.unwrap() == 0 { 1 } else { 0 }),
                    _ => return (None, Some(self.illegal_operation(Some(other)))),
                };

                (
                    Some(Value::NumberValue(Number::new(result)).set_context(self.context.clone())),
                    None,
                )
            }
            _ => (None, Some(self.illegal_operation(Some(other)))),
        }
    }

    pub fn is_true(&self) -> bool {
        self.value.unwrap() != 0
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

    pub fn as_string(&self) -> String {
        format!("{}", self.value.unwrap()).to_string()
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<number: {}>", self.value.unwrap_or_else(|| { 0 }))
    }
}

//     def subtractedBy(self, other):
//         if isinstance(other, Number):
//             return Number(self.value - other.value).setContext(self.context), None

//         else:
//             return None, Value.illegalOperation(self, other)

//     def multipliedBy(self, other):
//         if isinstance(other, Number):
//             return Number(self.value * other.value).setContext(self.context), None

//         else:
//             return None, Value.illegalOperation(self, other)

//     def dividedBy(self, other):
//         if isinstance(other, Number):
//             if other.value == 0:
//                 return None, RunTimeError(
//                     other.pos_start, other.pos_end, 'Division by zero',
//                     self.context
//                 )

//             return Number(self.value / other.value).setContext(self.context), None

//         else:
//             return None, Value.illegalOperation(self, other)

//     def poweredBy(self, other):
//         if isinstance(other, Number):
//             return Number(self.value ** other.value).setContext(self.context), None

//         else:
//             return None, Value.illegalOperation(self, other)

//     def getComparisonEq(self, other):
//         if isinstance(other, Number):
//             return Number(int(self.value == other.value)).setContext(self.context), None

//         else:
//             return None, Value.illegalOperation(self, other)

//     def getComparisonNe(self, other):
//         if isinstance(other, Number):
//             return Number(int(self.value != other.value)).setContext(self.context), None

//         else:
//             return None, Value.illegalOperation(self, other)

//     def getComparisonLt(self, other):
//         if isinstance(other, Number):
//             return Number(int(self.value < other.value)).setContext(self.context), None

//         else:
//             return None, Value.illegalOperation(self, other)

//     def getComparisonGt(self, other):
//         if isinstance(other, Number):
//             return Number(int(self.value > other.value)).setContext(self.context), None

//         else:
//             return None, Value.illegalOperation(self, other)

//     def getComparisonLte(self, other):
//         if isinstance(other, Number):
//             return Number(int(self.value <= other.value)).setContext(self.context), None

//         else:
//             return None, Value.illegalOperation(self, other)

//     def getComparisonGte(self, other):
//         if isinstance(other, Number):
//             return Number(int(self.value >= other.value)).setContext(self.context), None

//         else:
//             return None, Value.illegalOperation(self, other)

//     def andedBy(self, other):
//         if isinstance(other, Number):
//             return Number(int(self.value and other.value)).setContext(self.context), None

//         else:
//             return None, Value.illegalOperation(self, other)

//     def oredBy(self, other):
//         if isinstance(other, Number):
//             return Number(int(self.value or other.value)).setContext(self.context), None

//         else:
//             return None, Value.illegalOperation(self, other)

//     def notted(self):
//         return Number(1 if self.value == 0 else 0).setContext(self.context), None

//     def copy(self):
//         copy = Number(self.value)
//         copy.setPos(self.pos_start, self.pos_end)
//         copy.setContext(self.context)
//         return copy

//     def isTrue(self):
//         return self.value != 0

//     def __str__(self):
//         return f'{self.value}'

//     def __repr__(self):
//         return f'<number: {self.value}>'
