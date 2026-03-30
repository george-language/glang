use crate::{Number, Value};
use glang_attributes::StandardError;
use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub struct RuntimeResult {
    pub value: Rc<RefCell<Value>>,
    pub error: Option<StandardError>,
    pub func_return_value: Option<Rc<RefCell<Value>>>,
    pub loop_should_continue: bool,
    pub loop_should_break: bool,
}

impl RuntimeResult {
    pub fn new() -> Self {
        Self {
            value: Number::null_value(),
            error: None,
            func_return_value: None,
            loop_should_continue: false,
            loop_should_break: false,
        }
    }

    pub fn reset(&mut self) {
        self.value = Number::null_value();
        self.error = None;
        self.func_return_value = None;
        self.loop_should_continue = false;
        self.loop_should_break = false;
    }

    pub fn register(&mut self, result: RuntimeResult) -> Rc<RefCell<Value>> {
        self.error = result.error;
        self.func_return_value = result.func_return_value;
        self.loop_should_continue = result.loop_should_continue;
        self.loop_should_break = result.loop_should_break;

        result.value
    }

    pub fn success(&mut self, value: Rc<RefCell<Value>>) -> RuntimeResult {
        self.reset();
        self.value = value;

        self.clone()
    }

    pub fn success_return(&mut self, value: Rc<RefCell<Value>>) -> RuntimeResult {
        self.reset();
        self.func_return_value = Some(value);

        self.clone()
    }

    pub fn success_continue(&mut self) -> RuntimeResult {
        self.reset();
        self.loop_should_continue = true;

        self.clone()
    }

    pub fn success_break(&mut self) -> RuntimeResult {
        self.reset();
        self.loop_should_break = true;

        self.clone()
    }

    pub fn failure(&mut self, error: StandardError) -> RuntimeResult {
        self.reset();
        self.error = Some(error);

        self.clone()
    }

    pub fn should_return(&self) -> bool {
        self.error.is_some()
            || self.func_return_value.is_some()
            || self.loop_should_continue
            || self.loop_should_break
    }

    pub fn should_propagate(&self) -> bool {
        if let Some(err) = &self.error {
            return err.error_propagates;
        }

        false
    }
}
