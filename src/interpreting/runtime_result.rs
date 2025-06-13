use crate::{errors::standard_error::StandardError, values::value::Value};

#[derive(Clone)]
pub struct RuntimeResult {
    pub value: Option<Box<dyn Value>>,
    pub error: Option<StandardError>,
    pub func_return_value: Option<Box<dyn Value>>,
    pub loop_should_continue: bool,
    pub loop_should_break: bool,
}

impl RuntimeResult {
    pub fn new() -> Self {
        RuntimeResult {
            value: None,
            error: None,
            func_return_value: None,
            loop_should_continue: false,
            loop_should_break: false,
        }
    }

    pub fn reset(&mut self) {
        self.value = None;
        self.error = None;
        self.func_return_value = None;
        self.loop_should_continue = false;
        self.loop_should_break = false;
    }

    pub fn register(&mut self, result: RuntimeResult) -> Option<Box<dyn Value>> {
        self.error = result.error;
        self.func_return_value = result.func_return_value;
        self.loop_should_continue = result.loop_should_continue;
        self.loop_should_break = result.loop_should_break;

        result.value
    }

    pub fn success(&mut self, value: Option<Box<dyn Value>>) -> RuntimeResult {
        self.reset();
        self.value = value;

        self.clone()
    }

    pub fn success_return(&mut self, value: Option<Box<dyn Value>>) -> RuntimeResult {
        self.reset();
        self.func_return_value = value;

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

    pub fn failure(&mut self, error: Option<StandardError>) -> RuntimeResult {
        self.reset();
        self.error = error;

        self.clone()
    }

    pub fn should_return(&self) -> bool {
        self.error.is_some()
            || self.func_return_value.is_some()
            || self.loop_should_continue
            || self.loop_should_break
    }
}
