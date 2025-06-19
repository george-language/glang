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
    pub global_symbol_table: Rc<RefCell<SymbolTable>>,
    pub context: Option<Context>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl BuiltInFunction {
    pub fn new(name: String, global_symbol_table: Rc<RefCell<SymbolTable>>) -> Self {
        BuiltInFunction {
            name: name,
            global_symbol_table: global_symbol_table,
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
            "tonumber" => return self.execute_tonumber(args, &mut exec_context),
            "tostring" => return self.execute_tostring(args, &mut exec_context),
            "uhoh" => return self.execute_error(args, &mut exec_context),
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

    pub fn execute_tonumber(
        &self,
        args: &Vec<Box<Value>>,
        exec_ctx: &mut Context,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_and_populate_args(&vec!["value".to_string()], args, exec_ctx));

        if result.should_return() {
            return result;
        }

        let string_to_convert = args[0].clone();

        let value: f64 = match string_to_convert.as_ref() {
            Value::StringValue(string) => match string.as_string().parse() {
                Ok(number) => number,
                Err(e) => {
                    return result.failure(Some(StandardError::new(
                        format!("string couldn't be converted to number {}", e).to_string(),
                        string_to_convert.position_start().unwrap().clone(),
                        string_to_convert.position_end().unwrap().clone(),
                        Some(
                            "make sure the string is represented as a valid number like '1.0'"
                                .to_string(),
                        ),
                    )));
                }
            },
            _ => {
                return result.failure(Some(StandardError::new(
                    "expected type string".to_string(),
                    string_to_convert.position_start().unwrap().clone(),
                    string_to_convert.position_end().unwrap().clone(),
                    Some("add a string like '1.0' to convert to a number object".to_string()),
                )));
            }
        };

        result.success(Some(Number::from(value)))
    }

    pub fn execute_error(&self, args: &Vec<Box<Value>>, exec_ctx: &mut Context) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_and_populate_args(&vec!["msg".to_string()], args, exec_ctx));

        if result.should_return() {
            return result;
        }

        let error = args[0].clone();

        let message = match error.as_ref() {
            Value::StringValue(_) => error,
            _ => {
                return result.failure(Some(StandardError::new(
                    "expected type string".to_string(),
                    error.position_start().unwrap().clone(),
                    error.position_end().unwrap().clone(),
                    Some("add an error message".to_string()),
                )));
            }
        };

        result.failure(Some(StandardError::new(
            message.as_string(),
            message.position_start().unwrap().clone(),
            message.position_end().unwrap().clone(),
            None,
        )))
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

        let file_to_import = match import.as_ref() {
            Value::StringValue(file) => file.as_string(),
            _ => {
                return result.failure(Some(StandardError::new(
                    "expected type string".to_string(),
                    import.position_start().unwrap().clone(),
                    import.position_end().unwrap().clone(),
                    Some("add the '.glang' file you would like to import".to_string()),
                )));
            }
        };

        if !fs::exists(&file_to_import).is_ok() || !&file_to_import.ends_with(".glang") {
            return result.failure(Some(StandardError::new(
                "file doesn't exist or isn't valid".to_string(),
                import.position_start().unwrap().clone(),
                import.position_end().unwrap().clone(),
                Some("add the '.glang' file you would like to import".to_string()),
            )));
        }

        let mut contents = String::new();

        match fs::read_to_string(&file_to_import) {
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
        module_context.symbol_table = Some(self.global_symbol_table.clone());
        let module_result = interpreter.visit(ast.node.unwrap(), &mut module_context);

        if module_result.error.is_some() {
            return result.failure(module_result.error);
        }

        let symbols: Vec<(String, Option<Box<Value>>)> = module_context
            .symbol_table
            .as_ref()
            .unwrap()
            .borrow()
            .symbols
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        for (name, value) in symbols {
            self.global_symbol_table.borrow_mut().set(
                name.clone(),
                Some(
                    value
                        .unwrap()
                        .set_context(Some(exec_ctx.clone()))
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
