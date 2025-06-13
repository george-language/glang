use crate::{
    errors::standard_error::StandardError,
    interpreting::{context::Context, runtime_result::RuntimeResult},
    lexing::position::Position,
    values::value::Value,
};
use std::fmt::Display;

#[derive(Clone)]
pub struct Number {
    pub value: Option<String>,
    pub context: Option<Context>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl Number {
    pub fn new(&self, value: Option<String>) -> Self {
        let number = Number {
            value: value,
            context: None,
            pos_start: None,
            pos_end: None,
        };

        number
    }
}

impl Value for Number {
    fn position_start(&self) -> Option<Position> {
        self.pos_start.clone()
    }

    fn position_end(&self) -> Option<Position> {
        self.pos_end.clone()
    }

    fn added_to(&self, other: Box<dyn Value>) -> (Option<Box<dyn Value>>, Option<StandardError>) {
        if let Some(other_num) = other.as_any().downcast_ref::<Number>() {
            let left = self.value.as_ref().unwrap().parse::<isize>().unwrap();
            let right = other_num.value.as_ref().unwrap().parse::<isize>().unwrap();

            let sum = left + right;
            let new_number = Number {
                value: Some(sum.to_string()),
                context: self.context.clone(),
                pos_start: None,
                pos_end: None,
            };
            (Some(Box::new(new_number) as Box<dyn Value>), None)
        } else {
            (None, self.illegal_operation(Some(other)))
        }
    }

    fn illegal_operation(&self, other: Option<Box<dyn Value>>) -> Option<StandardError> {
        let mut other_value = &other;

        if other.is_none() {
            other_value = &Some(Box::new(self.to_owned()) as Box<dyn Value>);
        }

        Some(StandardError::new(
            "operation not allowed".to_string(),
            self.pos_start.clone().unwrap(),
            other.unwrap().position_end().unwrap(),
            Some("integers and floats only allow operations on each other".to_string()),
        ))
    }

    fn clone_box(&self) -> Box<dyn Value> {
        Box::new(self.clone())
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
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
