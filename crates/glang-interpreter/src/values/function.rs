use crate::{
    List, Str,
    context::Context,
    interpreter::Interpreter,
    runtime_result::RuntimeResult,
    symbol_table::SymbolTable,
    values::{number::Number, value::Value},
};
use glang_attributes::{Span, StandardError};
use glang_parser::{AstArena, NodeID};
use std::{
    cell::RefCell,
    env, fs,
    io::{Write, stdin, stdout},
    rc::Rc,
};

trait FunctionObject {
    fn generate_new_context(&self) -> Rc<RefCell<Context>>;

    fn check_args(&self, arg_names: &[String], args: &[Rc<RefCell<Value>>]) -> RuntimeResult;

    fn populate_args(
        &self,
        arg_names: &[String],
        args: &[Rc<RefCell<Value>>],
        expr_ctx: Rc<RefCell<Context>>,
    );

    fn check_and_populate_args(
        &self,
        arg_names: &[String],
        args: &[Rc<RefCell<Value>>],
        expr_ctx: Rc<RefCell<Context>>,
    ) -> RuntimeResult;
}

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
}

impl FunctionObject for Function {
    fn generate_new_context(&self) -> Rc<RefCell<Context>> {
        let parent_st = self.context.as_ref().unwrap().borrow().symbol_table.clone();
        let new_context = Context::new(
            Some(self.context.as_ref().unwrap().clone()),
            Some(self.span.clone()),
            Rc::new(RefCell::new(SymbolTable::new(Some(parent_st)))),
        );

        Rc::new(RefCell::new(new_context))
    }

    fn check_args(&self, arg_names: &[String], args: &[Rc<RefCell<Value>>]) -> RuntimeResult {
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

    fn populate_args(
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

    fn check_and_populate_args(
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
}

#[derive(Debug, Clone)]
pub struct BuiltInFunction {
    pub name: String,
    pub context: Option<Rc<RefCell<Context>>>,
    pub is_const: bool,
    pub span: Span,
}

impl BuiltInFunction {
    pub fn new(name: &str) -> Self {
        BuiltInFunction {
            name: name.to_string(),
            context: None,
            is_const: false,
            span: Span::empty(),
        }
    }

    pub fn from(name: &str) -> Rc<RefCell<Value>> {
        Rc::new(RefCell::new(Value::BuiltInFunction(BuiltInFunction::new(
            name,
        ))))
    }

    pub fn execute(&self, args: &[Rc<RefCell<Value>>]) -> RuntimeResult {
        let exec_context = self.generate_new_context();

        match self.name.as_str() {
            "bark" => self.execute_print(args, exec_context),
            "chew" => self.execute_input(args, exec_context),
            "dig" => self.execute_read(args, exec_context),
            "bury" => self.execute_write(args, exec_context),
            "copy" => self.execute_copy(args, exec_context),
            "clear" => self.execute_clear(args, exec_context),
            "tostring" => self.execute_tostring(args, exec_context),
            "tonumber" => self.execute_tonumber(args, exec_context),
            "length" => self.execute_length(args, exec_context),
            "uhoh" => self.execute_error(args, exec_context),
            "type" => self.execute_type(args, exec_context),
            "_env" => self.execute_env(args, exec_context),
            "split" => self.execute_split(args, exec_context),
            "round" => self.execute_round(args, exec_context),
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

        result.success(Number::null_value())
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
            Value::StringValue(ref string) => string.value.clone(),
            _ => {
                return result.failure(StandardError::new(
                    "expected type string",
                    message_arg.borrow().span(),
                    Some("add a message like 'Enter a number:' to get user input"),
                ));
            }
        };

        print!("{message}");

        let mut input = String::new();

        let _ = stdout().flush();

        stdin()
            .read_line(&mut input)
            .expect("did not enter a valid string");

        let input = input.trim_end();

        result.success(Str::from(&input))
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
            Value::StringValue(ref string) => string.value.clone(),
            _ => {
                return result.failure(StandardError::new(
                    "expected type string",
                    file_arg.borrow().span(),
                    Some("add a filename to read like 'test.txt'"),
                ));
            }
        };

        if fs::exists(&filename).is_err() {
            return result.failure(StandardError::new(
                "file doesn't exist",
                file_arg.borrow().span(),
                Some("add a filename to read like 'test.txt'"),
            ));
        }

        let mut contents = String::new();

        match fs::read_to_string(&filename) {
            Ok(extra) => contents.push_str(&extra),
            Err(_) => {
                return result.failure(StandardError::new(
                    "file contents couldn't be read properly",
                    file_arg.borrow().span(),
                    Some("add a UTF-8 encoded file to read"),
                ));
            }
        }

        result.success(Number::null_value())
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
            Value::StringValue(ref string) => string.value.clone(),
            _ => {
                return result.failure(StandardError::new(
                    "expected type string",
                    file_arg.borrow().span(),
                    Some("add a filename to write to like 'test.txt'"),
                ));
            }
        };

        let contents = match *contents_arg.borrow() {
            Value::StringValue(ref string) => string.value.clone(),
            _ => {
                return result.failure(StandardError::new(
                    "expected type string",
                    file_arg.borrow().span(),
                    Some("add the file contents to write into the file"),
                ));
            }
        };

        match fs::write(&filename, &contents) {
            Ok(_) => {}
            Err(_) => {
                return result.failure(StandardError::new(
                    "file contents couldn't be written properly",
                    file_arg.borrow().span(),
                    None,
                ));
            }
        }

        result.success(Number::null_value())
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

        result.success(Rc::new(RefCell::new(object_arg.borrow().clone()))) // we need to make a deep copy of the object
    }

    pub fn execute_clear(
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
        let span = object_arg.borrow().span();

        match *object_arg.borrow_mut() {
            Value::ListValue(ref mut v) => v.elements.clear(),
            Value::StringValue(ref mut v) => v.value.clear(),
            _ => {
                return result.failure(StandardError::new(
                    "expected type list or string",
                    span,
                    None,
                ));
            }
        }

        result.success(Number::null_value())
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

        result.success(Str::from(&args[0].borrow().as_string()))
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
            Value::StringValue(ref string) => match string.value.clone().parse() {
                Ok(number) => number,
                Err(e) => {
                    return result.failure(StandardError::new(
                        format!("string couldn't be converted to number {e}").as_str(),
                        string_to_convert.borrow().span(),
                        Some("ensure the string is represented as a valid number like '1.0'"),
                    ));
                }
            },
            _ => {
                return result.failure(StandardError::new(
                    "expected type string",
                    string_to_convert.borrow().span(),
                    Some("add a string like '1.0' to convert to a number object"),
                ));
            }
        };

        result.success(Number::from(value))
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
                return result.failure(StandardError::new(
                    "expected type string or list",
                    object_arg.borrow().span(),
                    None,
                ));
            }
        };

        result.success(Number::from(length))
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
                return result.failure(StandardError::new(
                    "expected type string",
                    error.borrow().span(),
                    Some("add an error message"),
                ));
            }
        };

        let mut error = StandardError::new(
            message.borrow().as_string().as_str(),
            message.borrow().span(),
            None,
        );
        error.error_propagates = true;

        result.failure(error)
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

        result.success(Str::from(&args[0].borrow().object_type().to_string()))
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
            Value::StringValue(ref var) => var.value.clone(),
            _ => {
                return result.failure(StandardError::new(
                    "expected type string",
                    env_arg.borrow().span(),
                    None,
                ));
            }
        };

        match env::var(&variable) {
            Ok(var) => result.success(Str::from(&var)),
            Err(_) => result.failure(StandardError::new(
                "unable to access environment variable",
                env_arg.borrow().span(),
                None,
            )),
        }
    }

    pub fn execute_split(
        &self,
        args: &[Rc<RefCell<Value>>],
        exec_ctx: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_and_populate_args(
            &["str".to_string(), "pattern".to_string()],
            args,
            exec_ctx,
        ));

        if result.should_return() {
            return result;
        }

        let string = args[0].clone();
        let pattern = args[1].clone();

        let elements = match (&*string.borrow(), &*pattern.borrow()) {
            (Value::StringValue(input), Value::StringValue(pat)) => input
                .value
                .split(&pat.value)
                .map(|s| Str::from(s))
                .collect::<Vec<_>>(),
            _ => {
                return result.failure(StandardError::new(
                    "expected type string",
                    string.borrow().span(),
                    None,
                ));
            }
        };

        result.success(List::from(elements))
    }

    pub fn execute_round(
        &self,
        args: &[Rc<RefCell<Value>>],
        exec_ctx: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        result.register(self.check_and_populate_args(&["num".to_string()], args, exec_ctx));

        if result.should_return() {
            return result;
        }

        let number = args[0].clone();

        match *number.borrow() {
            Value::NumberValue(ref num) => result.success(Number::from(num.value.round())),
            _ => {
                return result.failure(StandardError::new(
                    "expected type number",
                    number.borrow().span(),
                    None,
                ));
            }
        }
    }
}

impl FunctionObject for BuiltInFunction {
    fn generate_new_context(&self) -> Rc<RefCell<Context>> {
        let parent_st = self.context.as_ref().unwrap().borrow().symbol_table.clone();
        let new_context = Context::new(
            Some(self.context.as_ref().unwrap().clone()),
            Some(self.span.clone()),
            Rc::new(RefCell::new(SymbolTable::new(Some(parent_st)))),
        );

        Rc::new(RefCell::new(new_context))
    }

    fn check_args(&self, arg_names: &[String], args: &[Rc<RefCell<Value>>]) -> RuntimeResult {
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

    fn populate_args(
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

    fn check_and_populate_args(
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
}
