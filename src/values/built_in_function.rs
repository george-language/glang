use crate::{
    errors::standard_error::StandardError,
    interpreting::{
        context::Context, interpreter::Interpreter, runtime_result::RuntimeResult,
        symbol_table::SymbolTable,
    },
    lexing::position::Position,
    nodes::ast_node::AstNode,
    values::{number::Number, string::StringObj, value::Value},
};

#[derive(Debug, Clone)]
pub struct BuiltInFunction {
    pub name: String,
    pub context: Option<Context>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl BuiltInFunction {
    pub fn new(name: String) -> Self {
        BuiltInFunction {
            name: name,
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
        new_context.symbol_table = Some(SymbolTable::new(Some(Box::new(
            new_context
                .parent
                .as_ref()
                .unwrap()
                .symbol_table
                .as_ref()
                .unwrap()
                .clone(),
        ))));

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
        let mut exec_context = self.generate_new_context();

        match self.name.as_str() {
            "bark" => return self.execute_print(args, &mut exec_context),
            "type" => return self.execute_type(args, &mut exec_context),
            _ => panic!("CRITICAL ERROR: BUILT IN NAME IS NOT DEFINED"),
        };
    }

    pub fn execute_print(&self, args: &Vec<Box<Value>>, expr_ctx: &mut Context) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_and_populate_args(&vec!["value".to_string()], args, expr_ctx));

        if result.should_return() {
            return result;
        }

        println!("{}", args[0].as_string());

        result.success(Some(Number::null_value()))
    }

    pub fn execute_type(&self, args: &Vec<Box<Value>>, expr_ctx: &mut Context) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_and_populate_args(&vec!["value".to_string()], args, expr_ctx));

        if result.should_return() {
            return result;
        }

        result.success(Some(StringObj::from(
            format!("{}", args[0].object_type()).as_str(),
        )))
    }

    pub fn as_string(&self) -> String {
        format!("built-in-function: {}", self.name).to_string()
    }
}
