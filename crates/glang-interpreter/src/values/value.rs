use crate::{
    context::Context,
    values::{
        function::{BuiltInFunction, Function},
        list::List,
        number::Number,
        string::Str,
    },
};
use glang_attributes::{Position, Span, StandardError};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub enum Value {
    NumberValue(Number),
    ListValue(List),
    StringValue(Str),
    FunctionValue(Function),
    BuiltInFunction(BuiltInFunction),
}

impl Value {
    pub fn span(&self) -> Span {
        match self {
            Value::NumberValue(value) => value.span.clone(),
            Value::ListValue(value) => value.span.clone(),
            Value::StringValue(value) => value.span.clone(),
            Value::FunctionValue(value) => value.span.clone(),
            Value::BuiltInFunction(value) => value.span.clone(),
        }
    }

    pub fn position_start(&self) -> Position {
        match self {
            Value::NumberValue(value) => value.span.start.clone(),
            Value::ListValue(value) => value.span.start.clone(),
            Value::StringValue(value) => value.span.start.clone(),
            Value::FunctionValue(value) => value.span.start.clone(),
            Value::BuiltInFunction(value) => value.span.start.clone(),
        }
    }

    pub fn position_end(&self) -> Position {
        match self {
            Value::NumberValue(value) => value.span.end.clone(),
            Value::ListValue(value) => value.span.end.clone(),
            Value::StringValue(value) => value.span.end.clone(),
            Value::FunctionValue(value) => value.span.end.clone(),
            Value::BuiltInFunction(value) => value.span.end.clone(),
        }
    }

    pub fn set_span(&mut self, span: Span) {
        match self {
            Value::NumberValue(value) => value.span = span,
            Value::ListValue(value) => value.span = span,
            Value::StringValue(value) => value.span = span,
            Value::FunctionValue(value) => value.span = span,
            Value::BuiltInFunction(value) => value.span = span,
        }
    }

    pub fn set_context(&mut self, context: Option<Rc<RefCell<Context>>>) {
        match self {
            Value::NumberValue(value) => value.context = context,
            Value::ListValue(value) => value.context = context,
            Value::StringValue(value) => value.context = context,
            Value::FunctionValue(value) => value.context = context,
            Value::BuiltInFunction(value) => value.context = context,
        }
    }

    pub fn set_const(&mut self, is_const: bool) {
        match self {
            Value::NumberValue(value) => value.is_const = is_const,
            Value::ListValue(value) => value.is_const = is_const,
            Value::StringValue(value) => value.is_const = is_const,
            Value::FunctionValue(value) => value.is_const = is_const,
            Value::BuiltInFunction(value) => value.is_const = is_const,
        }
    }

    pub fn perform_operation(
        &mut self,
        operator: &str,
        other: Rc<RefCell<Value>>,
    ) -> Result<Rc<RefCell<Value>>, StandardError> {
        match self {
            Value::NumberValue(value) => value.perform_operation(operator, other),
            Value::ListValue(value) => value.perform_operation(operator, other),
            Value::StringValue(value) => value.perform_operation(operator, other),
            _ => Err(StandardError::new(
                format!("type doesn't support the '{operator}' operator").as_str(),
                self.span(),
                None,
            )),
        }
    }

    pub fn object_type(&self) -> &str {
        match self {
            Value::NumberValue(_) => "number",
            Value::ListValue(_) => "list",
            Value::StringValue(_) => "string",
            Value::FunctionValue(_) => "function",
            Value::BuiltInFunction(_) => "built-in-function",
        }
    }

    pub fn is_true(&self) -> bool {
        match self {
            Value::NumberValue(value) => value.value != 0.0,
            Value::ListValue(value) => value.elements.is_empty(),
            Value::StringValue(value) => value.value.is_empty(),
            Value::FunctionValue(value) => value.name.is_empty(),
            Value::BuiltInFunction(value) => value.name.is_empty(),
        }
    }

    pub fn is_const(&self) -> bool {
        match self {
            Value::NumberValue(value) => value.is_const,
            Value::ListValue(value) => value.is_const,
            Value::StringValue(value) => value.is_const,
            Value::FunctionValue(value) => value.is_const,
            Value::BuiltInFunction(value) => value.is_const,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Value::NumberValue(value) => value.as_string(),
            Value::ListValue(value) => value.as_string(),
            Value::StringValue(value) => value.as_string(),
            Value::FunctionValue(value) => value.as_string(),
            Value::BuiltInFunction(value) => value.as_string(),
        }
    }
}
