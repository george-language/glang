use crate::{
    context::Context,
    interpreter::Interpreter,
    runtime_result::RuntimeResult,
    symbol_table::SymbolTable,
    values::{number::Number, value::Value},
};
use glang_attributes::{Position, StandardError};
use glang_parser::AstNode;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub body_node: Box<AstNode>,
    pub arg_names: Rc<[String]>,
    pub should_auto_return: bool,
    pub context: Option<Rc<RefCell<Context>>>,
    pub is_const: bool,
    pub pos_start: Option<Rc<Position>>,
    pub pos_end: Option<Rc<Position>>,
}

impl Function {
    pub fn new(
        name: String,
        body_node: Box<AstNode>,
        arg_names: &[String],
        should_auto_return: bool,
    ) -> Self {
        Self {
            name,
            body_node,
            arg_names: Rc::from(arg_names),
            should_auto_return,
            context: None,
            is_const: false,
            pos_start: None,
            pos_end: None,
        }
    }

    pub fn generate_new_context(&self) -> Rc<RefCell<Context>> {
        let mut new_context = Context::new(
            self.name.clone(),
            Some(self.context.as_ref().unwrap().clone()),
            self.pos_start.clone(),
        );
        let parent_st = self
            .context
            .as_ref()
            .unwrap()
            .borrow()
            .symbol_table
            .as_ref()
            .unwrap()
            .clone();
        new_context.symbol_table = Some(Rc::new(RefCell::new(SymbolTable::new(Some(parent_st)))));

        Rc::new(RefCell::new(new_context))
    }

    pub fn check_args(&self, arg_names: &[String], args: &[Rc<RefCell<Value>>]) -> RuntimeResult {
        let mut result = RuntimeResult::new();

        if args.len() > arg_names.len() || args.len() < arg_names.len() {
            return result.failure(Some(StandardError::new(
                "invalid function call",
                self.pos_start.as_ref().unwrap().clone(),
                self.pos_end.as_ref().unwrap().clone(),
                Some(
                    format!(
                        "{} takes {} positional argument(s) but the program gave {}",
                        self.name,
                        arg_names.len(),
                        args.len()
                    )
                    .as_str(),
                ),
            )));
        }

        result.success(None)
    }

    pub fn populate_args(
        &self,
        arg_names: &[String],
        args: &[Rc<RefCell<Value>>],
        expr_ctx: Rc<RefCell<Context>>,
    ) {
        for i in 0..args.len() {
            let arg_name = arg_names[i].clone();
            let arg_value = args[i].clone();
            arg_value.borrow_mut().set_context(Some(expr_ctx.clone()));

            expr_ctx
                .borrow_mut()
                .symbol_table
                .as_mut()
                .unwrap()
                .borrow_mut()
                .set(arg_name.to_string(), Some(arg_value));
        }
    }

    pub fn check_and_populate_args(
        &self,
        arg_names: &[String],
        args: &[Rc<RefCell<Value>>],
        expr_ctx: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_args(arg_names, args));

        if result.should_return() {
            return result;
        }

        self.populate_args(arg_names, args, expr_ctx);

        result.success(None)
    }

    pub fn execute(&self, args: &[Rc<RefCell<Value>>]) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let mut interpreter = Interpreter::new();
        let exec_context = self.generate_new_context();

        result.register(self.check_and_populate_args(&self.arg_names, args, exec_context.clone()));

        if result.should_return() {
            return result;
        }

        let value =
            result.register(interpreter.visit(self.body_node.as_ref(), exec_context.clone()));

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
