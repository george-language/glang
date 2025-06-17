use std::{cell::RefCell, rc::Rc};

use crate::{
    errors::standard_error::StandardError,
    interpreting::{
        context::Context, interpreter::Interpreter, runtime_result::RuntimeResult,
        symbol_table::SymbolTable,
    },
    lexing::position::Position,
    nodes::ast_node::AstNode,
    values::{number::Number, value::Value},
};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub body_node: Box<AstNode>,
    pub arg_names: Vec<String>,
    pub should_auto_return: bool,
    pub context: Option<Context>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl Function {
    pub fn new(
        name: String,
        body_node: Box<AstNode>,
        arg_names: Vec<String>,
        should_auto_return: bool,
    ) -> Self {
        Function {
            name: name,
            body_node: body_node,
            arg_names: arg_names,
            should_auto_return: should_auto_return,
            context: None,
            pos_start: None,
            pos_end: None,
        }
    }

    pub fn generate_new_context(&self) -> Context {
        let mut new_context = Context::new(
            self.name.clone(),
            Some(Box::new(self.context.as_ref().unwrap().clone())),
            self.pos_start.clone(),
        );
        new_context.symbol_table = Some(Rc::new(RefCell::new(SymbolTable::new(Some(Box::new(
            new_context
                .parent
                .as_ref()
                .unwrap()
                .symbol_table
                .as_ref()
                .unwrap()
                .borrow()
                .clone(),
        ))))));

        new_context
    }

    pub fn check_args(&self, arg_names: &Vec<String>, args: &Vec<Box<Value>>) -> RuntimeResult {
        let mut result = RuntimeResult::new();

        if args.len() > arg_names.len() || args.len() < arg_names.len() {
            return result.failure(Some(StandardError::new(
                "invalid function call".to_string(),
                self.pos_start.as_ref().unwrap().clone(),
                self.pos_end.as_ref().unwrap().clone(),
                Some(
                    format!(
                        "{} takes {} positional argument(s) but the program gave {}",
                        self.name,
                        arg_names.len(),
                        args.len()
                    )
                    .to_string(),
                ),
            )));
        }

        result.success(None)
    }

    pub fn populate_args(
        &self,
        arg_names: &Vec<String>,
        args: &Vec<Box<Value>>,
        expr_ctx: &mut Context,
    ) {
        for i in 0..args.len() {
            let arg_name = arg_names[i].clone();
            let mut arg_value = args[i].clone();
            arg_value.set_context(Some(expr_ctx.clone()));

            expr_ctx
                .symbol_table
                .as_mut()
                .unwrap()
                .borrow_mut()
                .set(arg_name, Some(arg_value));
        }
    }

    pub fn check_and_populate_args(
        &self,
        arg_names: &Vec<String>,
        args: &Vec<Box<Value>>,
        expr_ctx: &mut Context,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_args(arg_names, args));

        if result.should_return() {
            return result;
        }

        self.populate_args(arg_names, args, expr_ctx);

        result.success(None)
    }

    pub fn execute(&self, args: &Vec<Box<Value>>) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let mut interpreter = Interpreter::new();
        let mut exec_context = self.generate_new_context();

        result.register(self.check_and_populate_args(&self.arg_names, args, &mut exec_context));

        if result.should_return() {
            return result;
        }

        let value = result.register(interpreter.visit(self.body_node.clone(), &mut exec_context));

        if result.should_return() && result.func_return_value.is_none() {
            return result;
        }

        let return_value = if self.should_auto_return { value } else { None }
            .or(result.func_return_value.clone())
            .or(Some(Number::null_value()));

        result.success(return_value)
    }

    pub fn as_string(&self) -> String {
        format!("function: {}", self.name).to_string()
    }
}
