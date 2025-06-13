use crate::{
    errors::standard_error::StandardError,
    interpreting::{context::Context, runtime_result::RuntimeResult},
    lexing::position::Position,
};
use std::{any::Any, fmt::Display};

pub trait Value: Display {
    fn position_start(&self) -> Option<Position>;
    fn position_end(&self) -> Option<Position>;

    fn added_to(&self, other: Box<dyn Value>) -> (Option<Box<dyn Value>>, Option<StandardError>);
    fn subtracted_by(
        &self,
        other: Box<dyn Value>,
    ) -> (Option<Box<dyn Value>>, Option<StandardError>);
    fn multiplied_by(
        &self,
        other: Box<dyn Value>,
    ) -> (Option<Box<dyn Value>>, Option<StandardError>);
    fn divided_by(&self, other: Box<dyn Value>) -> (Option<Box<dyn Value>>, Option<StandardError>);
    fn powered_by(&self, other: Box<dyn Value>) -> (Option<Box<dyn Value>>, Option<StandardError>);
    fn get_comparison_eq(
        &self,
        other: Box<dyn Value>,
    ) -> (Option<Box<dyn Value>>, Option<StandardError>);
    fn get_comparison_ne(
        &self,
        other: Box<dyn Value>,
    ) -> (Option<Box<dyn Value>>, Option<StandardError>);
    fn get_comparison_lt(
        &self,
        other: Box<dyn Value>,
    ) -> (Option<Box<dyn Value>>, Option<StandardError>);
    fn get_comparison_gt(
        &self,
        other: Box<dyn Value>,
    ) -> (Option<Box<dyn Value>>, Option<StandardError>);
    fn get_comparison_lte(
        &self,
        other: Box<dyn Value>,
    ) -> (Option<Box<dyn Value>>, Option<StandardError>);
    fn get_comparison_gte(
        &self,
        other: Box<dyn Value>,
    ) -> (Option<Box<dyn Value>>, Option<StandardError>);
    fn anded_by(&self, other: Box<dyn Value>) -> (Option<Box<dyn Value>>, Option<StandardError>);
    fn ored_by(&self, other: Box<dyn Value>) -> (Option<Box<dyn Value>>, Option<StandardError>);
    fn notted(&self) -> (Option<Box<dyn Value>>, Option<StandardError>);
    fn is_true(&self) -> (Option<Box<dyn Value>>, Option<StandardError>);
    fn execute(&self, args: Vec<Box<dyn Value>>) -> RuntimeResult {
        RuntimeResult::new().failure(self.illegal_operation(None))
    }
    fn illegal_operation(&self, other: Option<Box<dyn Value>>) -> Option<StandardError>;
    fn clone_box(&self) -> Box<dyn Value>;
    fn as_any(&self) -> &dyn Any;
}

impl Clone for Box<dyn Value> {
    fn clone(&self) -> Box<dyn Value> {
        self.clone_box()
    }
}
