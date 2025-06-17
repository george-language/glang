use std::{cell::RefCell, fs, rc::Rc};

use crate::{
    errors::standard_error::StandardError,
    interpreting::{
        context::Context, interpreter::Interpreter, runtime_result::RuntimeResult,
        symbol_table::SymbolTable,
    },
    lexing::{lexer::Lexer, position::Position},
    parsing::parser::Parser,
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
        exec_ctx: &mut Context,
    ) {
        for i in 0..args.len() {
            let arg_name = arg_names[i].clone();
            let mut arg_value = args[i].clone();
            arg_value.set_context(Some(exec_ctx.clone()));

            exec_ctx
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
        exec_ctx: &mut Context,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_args(arg_names, args));

        if result.should_return() {
            return result;
        }

        self.populate_args(arg_names, args, exec_ctx);

        result.success(None)
    }

    pub fn execute(&self, args: &Vec<Box<Value>>) -> RuntimeResult {
        let mut exec_context = self.generate_new_context();

        match self.name.as_str() {
            "bark" => return self.execute_print(args, &mut exec_context),
            "tostring" => return self.execute_tostring(args, &mut exec_context),
            "type" => return self.execute_type(args, &mut exec_context),
            "fetch" => return self.execute_import(args, &mut exec_context),
            _ => panic!("CRITICAL ERROR: BUILT IN NAME IS NOT DEFINED"),
        };
    }

    pub fn execute_print(&self, args: &Vec<Box<Value>>, exec_ctx: &mut Context) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_and_populate_args(&vec!["value".to_string()], args, exec_ctx));

        if result.should_return() {
            return result;
        }

        println!("{}", args[0].as_string());

        result.success(Some(Number::null_value()))
    }

    pub fn execute_tostring(
        &self,
        args: &Vec<Box<Value>>,
        exec_ctx: &mut Context,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_and_populate_args(&vec!["value".to_string()], args, exec_ctx));

        if result.should_return() {
            return result;
        }

        result.success(Some(StringObj::from(args[0].as_string().as_str())))
    }

    pub fn execute_type(&self, args: &Vec<Box<Value>>, exec_ctx: &mut Context) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_and_populate_args(&vec!["value".to_string()], args, exec_ctx));

        if result.should_return() {
            return result;
        }

        result.success(Some(StringObj::from(
            format!("{}", args[0].object_type()).as_str(),
        )))
    }

    pub fn execute_import(&self, args: &Vec<Box<Value>>, exec_ctx: &mut Context) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_and_populate_args(&vec!["file".to_string()], args, exec_ctx));

        if result.should_return() {
            return result;
        }

        let import = args[0].clone();

        if import.object_type().to_string() != "string".to_string() {
            return result.failure(Some(StandardError::new(
                "expected type string".to_string(),
                import.position_start().unwrap().clone(),
                import.position_end().unwrap().clone(),
                Some("add the '.glang' file you would like to import".to_string()),
            )));
        }

        if !fs::exists(import.as_string()).is_ok() || !import.as_string().ends_with(".glang") {
            return result.failure(Some(StandardError::new(
                "file doesn't exist or isn't valid".to_string(),
                import.position_start().unwrap().clone(),
                import.position_end().unwrap().clone(),
                Some("add the '.glang' file you would like to import".to_string()),
            )));
        }

        let mut contents = String::new();

        match fs::read_to_string(import.as_string()) {
            Ok(extra) => contents.push_str(&extra),
            Err(_) => {
                return result.failure(Some(StandardError::new(
                    "file contents couldn't be read properly".to_string(),
                    import.position_start().unwrap().clone(),
                    import.position_end().unwrap().clone(),
                    Some("add a UTF-8 encoded '.glang' file you would like to import".to_string()),
                )));
            }
        }

        let mut lexer = Lexer::new(import.position_start().unwrap().filename, contents.clone());
        let (tokens, error) = lexer.make_tokens();

        if error.is_some() {
            return result.failure(error);
        }

        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

        if ast.error.is_some() {
            return result.failure(ast.error);
        }

        let mut interpreter = Interpreter::new();
        let mut module_context = Context::new("<module>".to_string(), None, None);
        module_context.symbol_table = Some(interpreter.global_symbol_table.clone());
        let module_result = interpreter.visit(ast.node.unwrap(), &mut module_context);

        if module_result.error.is_some() {
            return result.failure(module_result.error);
        }

        let exec_context_copy = exec_ctx.clone();

        for (name, value) in module_context.symbol_table.unwrap().borrow().symbols.iter() {
            println!("{name}");
            exec_ctx
                .parent
                .as_mut()
                .unwrap()
                .symbol_table
                .as_mut()
                .unwrap()
                .borrow_mut()
                .set(
                    name.clone(),
                    Some(
                        value
                            .clone()
                            .unwrap()
                            .set_context(Some(exec_context_copy.clone()))
                            .set_position(self.pos_start.clone(), self.pos_end.clone()),
                    ),
                );
        }

        result.success(Some(Number::null_value()))
    }

    pub fn as_string(&self) -> String {
        format!("built-in-function: {}", self.name).to_string()
    }
}
