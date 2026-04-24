use crate::{
    context::Context,
    interpreter::Interpreter,
    runtime_result::RuntimeResult,
    symbol_table::SymbolTable,
    values::{number::Number, value::Value},
};
use glang_attributes::{Span, StandardError};
use glang_parser::{AstArena, NodeID};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub body_node: NodeID,
    pub arena: AstArena, // functions must own their own arena
    pub arg_names: Rc<[String]>,
    pub should_auto_return: bool,
    pub context: Option<Rc<RefCell<Context>>>,
    pub is_const: bool,
    pub span: Span,
}

impl Function {
    pub fn new(
        name: String,
        body_node: NodeID,
        arena: AstArena,
        arg_names: &[String],
        should_auto_return: bool,
    ) -> Self {
        Self {
            name,
            body_node,
            arena,
            arg_names: Rc::from(arg_names),
            should_auto_return,
            context: None,
            is_const: false,
            span: Span::empty(),
        }
    }

    pub fn generate_new_context(&self) -> Rc<RefCell<Context>> {
        let parent_st = self.context.as_ref().unwrap().borrow().symbol_table.clone();
        let new_context = Context::new(
            self.context.clone(),
            Some(self.span.clone()),
            Rc::new(RefCell::new(SymbolTable::new(Some(parent_st)))),
        );

        Rc::new(RefCell::new(new_context))
    }

    pub fn check_args(&self, arg_names: &[String], args: &[Rc<RefCell<Value>>]) -> RuntimeResult {
        let mut result = RuntimeResult::new();

        if args.len() > arg_names.len() || args.len() < arg_names.len() {
            return result.failure(StandardError::new(
                "invalid function call",
                self.span.clone(),
                Some(
                    format!(
                        "{} takes {} argument{} but the program gave {}",
                        self.name,
                        arg_names.len(),
                        if arg_names.len() > 1 { "s" } else { "" },
                        args.len()
                    )
                    .as_str(),
                ),
            ));
        }

        result.success(Number::null_value())
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
                .borrow_mut()
                .set(arg_name.to_string(), arg_value);
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

        result.success(Number::null_value())
    }

    pub fn execute(
        &self,
        args: &[Rc<RefCell<Value>>],
        interpreter: &mut Interpreter,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let exec_context = self.generate_new_context();

        result.register(self.check_and_populate_args(&self.arg_names, args, exec_context.clone()));

        if result.should_return() {
            return result;
        }

        let value =
            result.register(interpreter.visit(self.body_node, &self.arena, exec_context.clone()));

        if result.should_return() && result.func_return_value.is_none() {
            return result;
        }

        let return_value: Option<Rc<RefCell<Value>>> = if self.should_auto_return {
            Some(value)
        } else {
            None
        }
        .or(result.func_return_value.clone())
        .or(Some(Number::null_value()));

        result.success(return_value.unwrap())
    }

    pub fn as_string(&self) -> String {
        format!("function: {}", self.name).to_string()
    }
}
