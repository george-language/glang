use crate::{
    context::Context,
    interpreter::Interpreter,
    runtime_result::RuntimeResult,
    symbol_table::SymbolTable,
    values::{number::Number, string::Str, value::Value},
};
use glang_attributes::{Position, StandardError};
use glang_lexer::Lexer;
use glang_parser::Parser;
use std::{
    cell::RefCell,
    collections::HashMap,
    env, fs,
    io::{Write, stdin, stdout},
    rc::Rc,
};

#[derive(Debug, Clone)]
pub struct BuiltInFunction {
    pub name: String,
    pub context: Option<Rc<RefCell<Context>>>,
    pub is_const: bool,
    pub pos_start: Option<Rc<Position>>,
    pub pos_end: Option<Rc<Position>>,
}

impl BuiltInFunction {
    pub fn new(name: &str) -> Self {
        BuiltInFunction {
            name: name.to_string(),
            context: None,
            is_const: false,
            pos_start: None,
            pos_end: None,
        }
    }

    pub fn from(name: &str) -> Rc<RefCell<Value>> {
        Rc::new(RefCell::new(Value::BuiltInFunction(BuiltInFunction::new(
            name,
        ))))
    }

    pub fn generate_new_context(&self) -> Rc<RefCell<Context>> {
        let mut new_context = Context::new(
            self.name.clone(),
            Some(self.context.as_ref().unwrap().clone()),
            self.pos_start.clone(),
            None,
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
        exec_ctx: Rc<RefCell<Context>>,
    ) {
        for i in 0..args.len() {
            let arg_name = arg_names[i].clone();
            let arg_value = args[i].clone();
            arg_value.borrow_mut().set_context(Some(exec_ctx.clone()));

            exec_ctx
                .borrow_mut()
                .symbol_table
                .as_mut()
                .unwrap()
                .borrow_mut()
                .set(arg_name, Some(arg_value));
        }
    }

    pub fn check_and_populate_args(
        &self,
        arg_names: &[String],
        args: &[Rc<RefCell<Value>>],
        exec_ctx: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_args(arg_names, args));

        if result.should_return() {
            return result;
        }

        self.populate_args(arg_names, args, exec_ctx);

        result.success(None)
    }

    pub fn execute(&self, args: &[Rc<RefCell<Value>>]) -> RuntimeResult {
        let exec_context = self.generate_new_context();

        match self.name.as_str() {
            "bark" => self.execute_print(args, exec_context),
            "chew" => self.execute_input(args, exec_context),
            "dig" => self.execute_read(args, exec_context),
            "bury" => self.execute_write(args, exec_context),
            "copy" => self.execute_copy(args, exec_context),
            "tostring" => self.execute_tostring(args, exec_context),
            "tonumber" => self.execute_tonumber(args, exec_context),
            "length" => self.execute_length(args, exec_context),
            "uhoh" => self.execute_error(args, exec_context),
            "type" => self.execute_type(args, exec_context),
            "run" => self.execute_exec(args, exec_context),
            "_env" => self.execute_env(args, exec_context),
            _ => panic!("CRITICAL ERROR: BUILT IN NAME IS NOT DEFINED"),
        }
    }

    pub fn execute_print(
        &self,
        args: &[Rc<RefCell<Value>>],
        exec_ctx: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_and_populate_args(&["value".to_string()], args, exec_ctx));

        if result.should_return() {
            return result;
        }

        println!("{}", args[0].borrow().as_string());

        result.success(Some(Number::null_value()))
    }

    pub fn execute_input(
        &self,
        args: &[Rc<RefCell<Value>>],
        exec_ctx: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_and_populate_args(&["msg".to_string()], args, exec_ctx));

        if result.should_return() {
            return result;
        }

        let message_arg = args[0].clone();

        let message = match *message_arg.borrow() {
            Value::StringValue(ref string) => string.as_string(),
            _ => {
                return result.failure(Some(StandardError::new(
                    "expected type string",
                    message_arg.borrow().position_start().unwrap().clone(),
                    message_arg.borrow().position_end().unwrap().clone(),
                    Some("add a message like 'Enter a number:' to get user input"),
                )));
            }
        };

        print!("{message}");

        let mut input = String::new();

        let _ = stdout().flush();

        stdin()
            .read_line(&mut input)
            .expect("did not enter a valid string");

        result.success(Some(Str::from(input.trim())))
    }

    pub fn execute_read(
        &self,
        args: &[Rc<RefCell<Value>>],
        exec_ctx: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_and_populate_args(&["file".to_string()], args, exec_ctx));

        if result.should_return() {
            return result;
        }

        let file_arg = args[0].clone();

        let filename = match *file_arg.borrow() {
            Value::StringValue(ref string) => string.as_string(),
            _ => {
                return result.failure(Some(StandardError::new(
                    "expected type string",
                    file_arg.borrow().position_start().unwrap().clone(),
                    file_arg.borrow().position_end().unwrap().clone(),
                    Some("add a filename to read like 'test.txt'"),
                )));
            }
        };

        if fs::exists(&filename).is_err() {
            return result.failure(Some(StandardError::new(
                "file doesn't exist",
                file_arg.borrow().position_start().unwrap().clone(),
                file_arg.borrow().position_end().unwrap().clone(),
                Some("add a filename to read like 'test.txt'"),
            )));
        }

        let mut contents = String::new();

        match fs::read_to_string(&filename) {
            Ok(extra) => contents.push_str(&extra),
            Err(_) => {
                return result.failure(Some(StandardError::new(
                    "file contents couldn't be read properly",
                    file_arg.borrow().position_start().unwrap().clone(),
                    file_arg.borrow().position_end().unwrap().clone(),
                    Some("add a UTF-8 encoded file you would like to read"),
                )));
            }
        }

        result.success(Some(Str::from(contents.as_str())))
    }

    pub fn execute_write(
        &self,
        args: &[Rc<RefCell<Value>>],
        exec_ctx: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_and_populate_args(
            &["file".to_string(), "contents".to_string()],
            args,
            exec_ctx,
        ));

        if result.should_return() {
            return result;
        }

        let file_arg = args[0].clone();
        let contents_arg = args[1].clone();

        let filename = match *file_arg.borrow() {
            Value::StringValue(ref string) => string.as_string(),
            _ => {
                return result.failure(Some(StandardError::new(
                    "expected type string",
                    file_arg.borrow().position_start().unwrap().clone(),
                    file_arg.borrow().position_end().unwrap().clone(),
                    Some("add a filename to write to like 'test.txt'"),
                )));
            }
        };

        let contents = match *contents_arg.borrow() {
            Value::StringValue(ref string) => string.as_string(),
            _ => {
                return result.failure(Some(StandardError::new(
                    "expected type string",
                    file_arg.borrow().position_start().unwrap().clone(),
                    file_arg.borrow().position_end().unwrap().clone(),
                    Some("add the file contents to write into the file"),
                )));
            }
        };

        match fs::write(&filename, &contents) {
            Ok(_) => {}
            Err(_) => {
                return result.failure(Some(StandardError::new(
                    "file contents couldn't be written properly",
                    file_arg.borrow().position_start().unwrap().clone(),
                    file_arg.borrow().position_end().unwrap().clone(),
                    None,
                )));
            }
        }

        result.success(Some(Number::null_value()))
    }

    pub fn execute_copy(
        &self,
        args: &[Rc<RefCell<Value>>],
        exec_ctx: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_and_populate_args(&["value".to_string()], args, exec_ctx));

        if result.should_return() {
            return result;
        }

        let object_arg = args[0].clone();

        result.success(Some(Rc::new(RefCell::new(object_arg.borrow().clone())))) // we need to make a deep copy of the object
    }

    pub fn execute_tostring(
        &self,
        args: &[Rc<RefCell<Value>>],
        exec_ctx: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_and_populate_args(&["value".to_string()], args, exec_ctx));

        if result.should_return() {
            return result;
        }

        result.success(Some(Str::from(args[0].borrow().as_string().as_str())))
    }

    pub fn execute_tonumber(
        &self,
        args: &[Rc<RefCell<Value>>],
        exec_ctx: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_and_populate_args(&["value".to_string()], args, exec_ctx));

        if result.should_return() {
            return result;
        }

        let string_to_convert = args[0].clone();

        let value: f64 = match *string_to_convert.borrow() {
            Value::StringValue(ref string) => match string.as_string().parse() {
                Ok(number) => number,
                Err(e) => {
                    return result.failure(Some(StandardError::new(
                        format!("string couldn't be converted to number {e}").as_str(),
                        string_to_convert.borrow().position_start().unwrap().clone(),
                        string_to_convert.borrow().position_end().unwrap().clone(),
                        Some("make sure the string is represented as a valid number like '1.0'"),
                    )));
                }
            },
            _ => {
                return result.failure(Some(StandardError::new(
                    "expected type string",
                    string_to_convert.borrow().position_start().unwrap().clone(),
                    string_to_convert.borrow().position_end().unwrap().clone(),
                    Some("add a string like '1.0' to convert to a number object"),
                )));
            }
        };

        result.success(Some(Number::from(value)))
    }

    pub fn execute_length(
        &self,
        args: &[Rc<RefCell<Value>>],
        exec_ctx: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_and_populate_args(&["value".to_string()], args, exec_ctx));

        if result.should_return() {
            return result;
        }

        let object_arg = args[0].clone();

        let length: f64 = match *object_arg.borrow() {
            Value::StringValue(ref value) => value.value.len() as f64,
            Value::ListValue(ref value) => value.elements.len() as f64,
            _ => {
                return result.failure(Some(StandardError::new(
                    "expected type string or list",
                    object_arg.borrow().position_start().unwrap().clone(),
                    object_arg.borrow().position_end().unwrap().clone(),
                    None,
                )));
            }
        };

        result.success(Some(Number::from(length)))
    }

    pub fn execute_error(
        &self,
        args: &[Rc<RefCell<Value>>],
        exec_ctx: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_and_populate_args(&["msg".to_string()], args, exec_ctx));

        if result.should_return() {
            return result;
        }

        let error = args[0].clone();

        let message = match *error.borrow() {
            Value::StringValue(_) => error.clone(),
            _ => {
                return result.failure(Some(StandardError::new(
                    "expected type string",
                    error.borrow().position_start().unwrap(),
                    error.borrow().position_end().unwrap(),
                    Some("add an error message"),
                )));
            }
        };

        result.failure(Some(StandardError::new(
            message.borrow().as_string().as_str(),
            message.borrow().position_start().unwrap().clone(),
            message.borrow().position_end().unwrap().clone(),
            None,
        )))
    }

    pub fn execute_type(
        &self,
        args: &[Rc<RefCell<Value>>],
        exec_ctx: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_and_populate_args(&["value".to_string()], args, exec_ctx));

        if result.should_return() {
            return result;
        }

        result.success(Some(Str::from(
            args[0].borrow().object_type().to_string().as_str(),
        )))
    }

    pub fn execute_exec(
        &self,
        args: &[Rc<RefCell<Value>>],
        exec_ctx: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_and_populate_args(&["code".to_string()], args, exec_ctx));

        if result.should_return() {
            return result;
        }

        let code_arg = args[0].clone();

        let code = match *code_arg.borrow() {
            Value::StringValue(ref glang) => glang.as_string(),
            _ => {
                return result.failure(Some(StandardError::new(
                    "expected type string",
                    code_arg.borrow().position_start().unwrap().clone(),
                    code_arg.borrow().position_end().unwrap().clone(),
                    Some("add the glang code you would like to execute"),
                )));
            }
        };

        let mut lexer = Lexer::new(
            &code_arg.borrow().position_start().unwrap().filename,
            code.clone(),
        );
        let token_result = lexer.make_tokens();

        if token_result.is_err() {
            return result.failure(token_result.err());
        }

        let mut parser = Parser::new(&token_result.ok().unwrap());
        let ast = parser.parse();

        if ast.error.is_some() {
            return result.failure(ast.error);
        }

        let mut interpreter = Interpreter::new(None, Rc::new(RefCell::new(HashMap::new())));
        let external_context = Rc::new(RefCell::new(Context::new(
            "<exec>".to_string(),
            None,
            None,
            None,
        )));
        external_context.borrow_mut().symbol_table = Some(interpreter.global_symbol_table.clone());

        if !cfg!(feature = "no-std") {
            interpreter.preload_standard_library(external_context.clone());
        }

        let external_result =
            interpreter.visit(ast.node.unwrap().as_ref(), external_context.clone());

        if external_result.error.is_some() {
            return result.failure(external_result.error);
        }

        result.success(Some(Number::null_value()))
    }

    pub fn execute_env(
        &self,
        args: &[Rc<RefCell<Value>>],
        exec_ctx: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_and_populate_args(&["var".to_string()], args, exec_ctx));

        if result.should_return() {
            return result;
        }

        let env_arg = args[0].clone();

        let variable = match *env_arg.borrow() {
            Value::StringValue(ref var) => var.as_string(),
            _ => {
                return result.failure(Some(StandardError::new(
                    "expected type string",
                    env_arg.borrow().position_start().unwrap().clone(),
                    env_arg.borrow().position_end().unwrap().clone(),
                    Some("add the glang code you would like to execute"),
                )));
            }
        };

        match env::var(&variable) {
            Ok(var) => result.success(Some(Str::from(&var))),
            Err(_) => result.failure(Some(StandardError::new(
                "unable to access environment variable",
                env_arg.borrow().position_start().unwrap().clone(),
                env_arg.borrow().position_end().unwrap().clone(),
                None,
            ))),
        }
    }

    pub fn as_string(&self) -> String {
        format!("built-in-function: {}", self.name).to_string()
    }
}
