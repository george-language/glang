use crate::{
    context::Context,
    runtime_result::RuntimeResult,
    symbol_table::SymbolTable,
    values::{
        built_in_function::BuiltInFunction, function::Function, list::List, number::Number,
        string::Str, value::Value,
    },
};
use glang_attributes::StandardError;
use glang_lexer::{Lexer, TokenType};
use glang_parser::nodes::{
    ast_node::AstNode, binary_operator_node::BinaryOperatorNode, break_node::BreakNode,
    call_node::CallNode, const_assign_node::ConstAssignNode, continue_node::ContinueNode,
    for_node::ForNode, function_definition_node::FunctionDefinitionNode, if_node::IfNode,
    import_node::ImportNode, list_node::ListNode, number_node::NumberNode, return_node::ReturnNode,
    string_node::StringNode, try_except_node::TryExceptNode,
    unary_operator_node::UnaryOperatorNode, variable_access_node::VariableAccessNode,
    variable_assign_node::VariableAssignNode, while_node::WhileNode,
};
use glang_parser::{Parser, nodes::variable_reassign_node::VariableRessignNode};
use std::{
    cell::RefCell,
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};

pub struct Interpreter {
    pub global_symbol_table: Rc<RefCell<SymbolTable>>,
    pub precached_std_lib: Option<Rc<RefCell<SymbolTable>>>,
    imported_modules: Rc<RefCell<HashMap<PathBuf, Rc<RefCell<SymbolTable>>>>>,
}

impl Interpreter {
    pub fn new(
        precached_std_lib: Option<Rc<RefCell<SymbolTable>>>,
        imported_modules: Rc<RefCell<HashMap<PathBuf, Rc<RefCell<SymbolTable>>>>>,
    ) -> Self {
        let interpreter = Self {
            global_symbol_table: Rc::new(RefCell::new(SymbolTable::new(None))),
            precached_std_lib,
            imported_modules,
        };

        let builtins = [
            "bark", "chew", "dig", "bury", "copy", "tostring", "tonumber", "length", "uhoh",
            "type", "run", "_env",
        ];

        for builtin in &builtins {
            interpreter
                .global_symbol_table
                .borrow_mut()
                .set(builtin.to_string(), Some(BuiltInFunction::from(builtin)));
        }

        interpreter
    }

    pub fn preload_standard_library(&mut self, context: Rc<RefCell<Context>>) {
        if let Some(e) = self.evaluate(
            "fetch _env(\"GLANG_STD\") + \"/fundamental/lib.glang\";",
            context.clone(),
        ) {
            println!("{}", e);

            return;
        }

        self.precached_std_lib = Some(context.borrow().symbol_table.as_ref().unwrap().clone());
    }

    pub fn evaluate(&mut self, src: &str, context: Rc<RefCell<Context>>) -> Option<StandardError> {
        let mut lexer = Lexer::new(Path::new("<eval>"), src.to_string());
        let token_result = lexer.make_tokens();

        if token_result.is_err() {
            return token_result.err();
        }

        let mut parser = Parser::new(&token_result.ok().unwrap());
        let ast = parser.parse();

        if ast.error.is_some() {
            return ast.error;
        }

        let result = self.visit(ast.node.unwrap().as_ref(), context);
        result.error
    }

    pub fn visit(&mut self, node: &AstNode, context: Rc<RefCell<Context>>) -> RuntimeResult {
        match node {
            AstNode::List(node) => self.visit_list_node(node, context),
            AstNode::Number(node) => self.visit_number_node(node, context),
            AstNode::Strings(node) => self.visit_string_node(node, context),
            AstNode::VariableAssign(node) => self.visit_variable_assign_node(node, context),
            AstNode::VariableReassign(node) => self.visit_variable_reassign_node(node, context),
            AstNode::ConstAssign(node) => self.visit_const_assign_node(node, context),
            AstNode::VariableAccess(node) => self.visit_variable_access_node(node, context),
            AstNode::If(node) => self.visit_if_node(node, context),
            AstNode::Import(node) => self.visit_import_node(node, context),
            AstNode::For(node) => self.visit_for_node(node, context),
            AstNode::While(node) => self.visit_while_node(node, context),
            AstNode::TryExcept(node) => self.visit_try_except_node(node, context),
            AstNode::FunctionDefinition(node) => self.visit_function_definition_node(node, context),
            AstNode::Call(node) => self.visit_call_node(node, context),
            AstNode::BinaryOperator(node) => self.visit_binary_operator_node(node, context),
            AstNode::UnaryOperator(node) => self.visit_unary_operator_node(node, context),
            AstNode::Return(node) => self.visit_return_node(node, context),
            AstNode::Continue(node) => self.visit_continue_node(node, context),
            AstNode::Break(node) => self.visit_break_node(node, context),
        }
    }

    fn visit_number_node(&self, node: &NumberNode, context: Rc<RefCell<Context>>) -> RuntimeResult {
        let value = Number::from(node.token.value.as_ref().unwrap().parse().unwrap());
        value.borrow_mut().set_context(Some(context.clone()));
        value
            .borrow_mut()
            .set_position(node.pos_start.clone(), node.pos_end.clone());

        RuntimeResult::new().success(Some(value))
    }

    fn visit_list_node(&mut self, node: &ListNode, context: Rc<RefCell<Context>>) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let mut elements: Vec<Rc<RefCell<Value>>> = Vec::new();

        for element in node.element_nodes.iter() {
            let element_result = result.register(self.visit(element.as_ref(), context.clone()));

            if result.should_return() {
                return result;
            }

            elements.push(element_result.unwrap());
        }

        let list = List::from(elements);
        list.borrow_mut().set_context(Some(context.clone()));
        list.borrow_mut()
            .set_position(node.pos_start.clone(), node.pos_end.clone());

        result.success(Some(list))
    }

    fn visit_string_node(
        &mut self,
        node: &StringNode,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let string = Str::from(node.token.value.as_ref().unwrap());
        string.borrow_mut().set_context(Some(context.clone()));
        string
            .borrow_mut()
            .set_position(node.pos_start.clone(), node.pos_end.clone());

        RuntimeResult::new().success(Some(string))
    }

    fn visit_variable_assign_node(
        &mut self,
        node: &VariableAssignNode,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let var_name = node.var_name_token.value.as_ref().unwrap().clone();

        let constant = context
            .borrow()
            .symbol_table
            .as_ref()
            .unwrap()
            .borrow()
            .get(&var_name);

        if constant.is_some() && constant.unwrap().borrow().is_const() {
            return result.failure(Some(StandardError::new(
                "cannot reassign the value of a constant",
                node.pos_start.as_ref().unwrap().to_owned(),
                node.pos_end.as_ref().unwrap().to_owned(),
                None,
            )));
        }

        let mut value = result.register(self.visit(node.value_node.as_ref(), context.clone()));

        if result.should_return() {
            return result;
        }

        context
            .borrow_mut()
            .symbol_table
            .as_mut()
            .unwrap()
            .borrow_mut()
            .set(var_name, value.clone());

        result.success(value)
    }

    fn visit_variable_reassign_node(
        &mut self,
        node: &VariableRessignNode,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let var_name = node.var_name_token.value.as_ref().unwrap().clone();

        let constant = context
            .borrow()
            .symbol_table
            .as_ref()
            .unwrap()
            .borrow()
            .get(&var_name);

        if constant.is_some() && constant.unwrap().borrow().is_const() {
            return result.failure(Some(StandardError::new(
                "cannot reassign the value of a constant",
                node.pos_start.as_ref().unwrap().to_owned(),
                node.pos_end.as_ref().unwrap().to_owned(),
                None,
            )));
        }

        if context
            .borrow()
            .symbol_table
            .as_ref()
            .unwrap()
            .borrow()
            .get(&var_name)
            .is_none()
        {
            return result.failure(Some(StandardError::new(
                format!("variable name '{var_name}' is undefined").as_str(),
                node.pos_start.as_ref().unwrap().clone(),
                node.pos_end.as_ref().unwrap().clone(),
                Some("define a variable with the syntax 'obj <variable name> = <value>;'"),
            )));
        }

        let mut value = result.register(self.visit(node.value_node.as_ref(), context.clone()));

        if result.should_return() {
            return result;
        }

        context
            .borrow_mut()
            .symbol_table
            .as_mut()
            .unwrap()
            .borrow_mut()
            .set(var_name, value.clone());

        result.success(value)
    }

    fn visit_const_assign_node(
        &mut self,
        node: &ConstAssignNode,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let const_name = node.const_name_token.value.as_ref().unwrap().clone();

        let constant = context
            .borrow()
            .symbol_table
            .as_ref()
            .unwrap()
            .borrow()
            .get(&const_name);

        if constant.is_some() && constant.unwrap().borrow().is_const() {
            return result.failure(Some(StandardError::new(
                "cannot reassign the value of a constant",
                node.pos_start.as_ref().unwrap().to_owned(),
                node.pos_end.as_ref().unwrap().to_owned(),
                None,
            )));
        }

        let mut value = result.register(self.visit(node.value_node.as_ref(), context.clone()));

        if result.should_return() {
            return result;
        }

        // if the value we are accessing is not a constant, we copy that value in place
        if !value.as_ref().unwrap().borrow().is_const() {
            value = Some(Rc::new(RefCell::new(value.unwrap().borrow().clone())));
            value.as_ref().unwrap().borrow_mut().set_const(true); // now a constant, cause we cloned
        }

        context
            .borrow_mut()
            .symbol_table
            .as_mut()
            .unwrap()
            .borrow_mut()
            .set(const_name, value.clone());

        result.success(value)
    }

    fn visit_variable_access_node(
        &mut self,
        node: &VariableAccessNode,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let var_name = node.var_name_token.value.as_ref().unwrap();
        let mut value = context
            .borrow()
            .symbol_table
            .as_ref()
            .unwrap()
            .borrow()
            .get(var_name.as_str())
            .clone();

        if value.is_none() {
            return result.failure(Some(StandardError::new(
                format!("variable name '{var_name}' is undefined").as_str(),
                node.pos_start.as_ref().unwrap().clone(),
                node.pos_end.as_ref().unwrap().clone(),
                Some("define a variable with the syntax 'obj <variable name> = <value>;'"),
            )));
        }

        // if the value we are accessing is a constant, we copy the constant
        if value.as_ref().unwrap().borrow().is_const() {
            value = Some(Rc::new(RefCell::new(value.unwrap().borrow().clone())));
            value.as_ref().unwrap().borrow_mut().set_const(false); // no longer a constant, cause we cloned
        }

        // prevent recursion issues by borrowing already borrowed objects
        if let Some(v) = &mut value.as_mut().unwrap().try_borrow_mut().ok() {
            v.set_context(Some(context.clone()));
            v.set_position(node.pos_start.clone(), node.pos_end.clone());
        }

        result.success(value)
    }

    fn visit_if_node(&mut self, node: &IfNode, context: Rc<RefCell<Context>>) -> RuntimeResult {
        let mut result = RuntimeResult::new();

        for (condition, expr, should_return_null) in node.cases.iter() {
            let condition_value = result.register(self.visit(condition.as_ref(), context.clone()));

            if result.should_return() {
                return result;
            }

            let condition_value = condition_value.unwrap();

            if condition_value.borrow().is_true() {
                let expr_value = result.register(self.visit(expr.as_ref(), context.clone()));

                if result.should_return() {
                    return result;
                }

                return result.success(if *should_return_null {
                    Some(Number::null_value())
                } else {
                    expr_value
                });
            }
        }

        if node.else_case.is_some() {
            let (expr, should_return_null) = node.else_case.as_ref().unwrap().clone();
            let else_value = result.register(self.visit(expr.as_ref(), context.clone()));

            if result.should_return() {
                return result;
            }

            return result.success(if should_return_null {
                Some(Number::null_value())
            } else {
                else_value
            });
        }

        result.success(Some(Number::null_value()))
    }

    fn visit_for_node(&mut self, node: &ForNode, context: Rc<RefCell<Context>>) -> RuntimeResult {
        let mut result = RuntimeResult::new();

        let start_value = match *result
            .register(self.visit(node.start_value_node.as_ref(), context.clone()))
            .unwrap()
            .borrow()
        {
            Value::NumberValue(ref value) => Number::new(value.value),
            _ => {
                return result.failure(Some(StandardError::new(
                    "expected start value as number",
                    node.pos_start.as_ref().unwrap().clone(),
                    node.pos_end.as_ref().unwrap().clone(),
                    None,
                )));
            }
        };

        if result.should_return() {
            return result;
        }

        let end_value = match *result
            .register(self.visit(node.end_value_node.as_ref(), context.clone()))
            .unwrap()
            .borrow()
        {
            Value::NumberValue(ref value) => Number::new(value.value),
            _ => {
                return result.failure(Some(StandardError::new(
                    "expected end value as number",
                    node.pos_start.as_ref().unwrap().clone(),
                    node.pos_end.as_ref().unwrap().clone(),
                    None,
                )));
            }
        };

        if result.should_return() {
            return result;
        }

        let step_value: Number;

        if let Some(step_value_node) = node.step_value_node.as_ref() {
            step_value = match *result
                .register(self.visit(step_value_node, context.clone()))
                .unwrap()
                .borrow()
            {
                Value::NumberValue(ref value) => Number::new(value.value),
                _ => {
                    return result.failure(Some(StandardError::new(
                        "expected step value as number",
                        node.pos_start.as_ref().unwrap().clone(),
                        node.pos_end.as_ref().unwrap().clone(),
                        None,
                    )));
                }
            };

            if result.should_return() {
                return result;
            }
        } else {
            step_value = Number::new(1.0);
        }

        if step_value.value == 0.0 {
            return result.failure(Some(StandardError::new(
                "step value of a 'walk' loop cannot be 0",
                node.step_value_node
                    .as_ref()
                    .unwrap()
                    .position_start()
                    .unwrap(),
                node.step_value_node
                    .as_ref()
                    .unwrap()
                    .position_end()
                    .unwrap(),
                Some("use a step value like 'step = 1' to control how many iteration steps occur"),
            )));
        }

        let iterator_name = node.var_name_token.value.as_ref().unwrap().to_owned();
        let symbol_table = context.borrow().symbol_table.as_ref().unwrap().clone();

        let range: Vec<f64> = if step_value.value > 0.0 {
            (start_value.value as i64..end_value.value as i64)
                .step_by(step_value.value as usize)
                .map(|x| x as f64)
                .collect()
        } else {
            (end_value.value as i64 + 1..=start_value.value as i64)
                .rev()
                .step_by((-step_value.value) as usize)
                .map(|x| x as f64)
                .collect()
        };

        for i in range {
            symbol_table
                .borrow_mut()
                .set(iterator_name.clone(), Some(Number::from(i)));

            let _ = result.register(self.visit(node.body_node.as_ref(), context.clone()));

            if result.should_return() && !result.loop_should_continue && !result.loop_should_break {
                return result;
            }

            if result.loop_should_continue {
                continue;
            }

            if result.loop_should_break {
                break;
            }
        }

        result.success(Some(Number::null_value()))
    }

    fn visit_while_node(
        &mut self,
        node: &WhileNode,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();

        loop {
            let condition =
                result.register(self.visit(node.condition_node.as_ref(), context.clone()));

            if result.should_return() {
                return result;
            }

            let condition = condition.unwrap();

            if !condition.borrow().is_true() {
                break;
            }

            let _ = result.register(self.visit(node.body_node.as_ref(), context.clone()));

            if result.should_return() && !result.loop_should_continue && !result.loop_should_break {
                return result;
            }

            if result.loop_should_continue {
                continue;
            }

            if result.loop_should_break {
                break;
            }
        }

        result.success(Some(Number::null_value()))
    }

    fn visit_try_except_node(
        &mut self,
        node: &TryExceptNode,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();

        let _ = result.register(self.visit(node.try_body_node.as_ref(), context.clone()));
        let try_error = result.error.clone();

        if try_error.is_some() {
            let output_error = Str::from(&try_error.unwrap().text);
            output_error.borrow_mut().set_const(true);

            context
                .borrow_mut()
                .symbol_table
                .as_mut()
                .unwrap()
                .borrow_mut()
                .set(
                    node.error_name_token.value.to_owned().unwrap(),
                    Some(output_error),
                );

            let _ = result.register(self.visit(node.except_body_node.as_ref(), context));

            if result.error.is_some() {
                return result;
            }

            if result.should_return() {
                return result;
            }
        } else if result.should_return() {
            return result;
        }

        result.success(Some(Number::null_value()))
    }

    fn visit_import_node(
        &mut self,
        node: &ImportNode,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let import_value =
            result.register(self.visit(node.node_to_import.as_ref(), context.clone()));

        if result.should_return() {
            return result;
        }

        let import_value = import_value.unwrap();
        let importing_path = import_value
            .borrow()
            .position_start()
            .unwrap()
            .filename
            .clone();

        let importing_dir = Path::new(&importing_path)
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_path_buf();

        let file_to_import = importing_dir.join(PathBuf::from(match *import_value.borrow() {
            Value::StringValue(ref string) => string.as_string(),
            _ => {
                return result.failure(Some(StandardError::new(
                    "expected type string",
                    import_value.borrow().position_start().unwrap(),
                    import_value.borrow().position_end().unwrap(),
                    Some("add the '.glang' file to import"),
                )));
            }
        }));

        if !(file_to_import.exists() || !file_to_import.ends_with(".glang")) {
            return result.failure(Some(StandardError::new(
                "invalid import",
                import_value.borrow().position_start().unwrap(),
                import_value.borrow().position_end().unwrap(),
                Some("add the '.glang' file to import"),
            )));
        }

        if file_to_import == importing_path {
            return result.failure(Some(StandardError::new(
                "circular import",
                import_value.borrow().position_start().unwrap(),
                import_value.borrow().position_end().unwrap(),
                None,
            )));
        }

        // if we already have imported modules stored in memory, then use cached ones
        if let Some(cached_symtab) = self.imported_modules.borrow().get(&file_to_import) {
            let symbols: Vec<(String, Option<Rc<RefCell<Value>>>)> = cached_symtab
                .borrow()
                .symbols
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();

            for (name, value) in symbols {
                context
                    .borrow_mut()
                    .symbol_table
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .set(name, value);
            }

            return result.success(Some(Number::null_value()));
        }

        let mut contents = String::new();

        match fs::read_to_string(&file_to_import) {
            Ok(extra) => contents.push_str(&extra),
            Err(_) => {
                return result.failure(Some(StandardError::new(
                    &format!(
                        "file contents couldn't be read properly on {}",
                        file_to_import.to_string_lossy()
                    ),
                    import_value.borrow().position_start().unwrap(),
                    import_value.borrow().position_end().unwrap(),
                    Some("add a UTF-8 encoded '.glang' file to import"),
                )));
            }
        }

        let mut lexer = Lexer::new(&file_to_import, contents);
        let token_result = lexer.make_tokens();

        if token_result.is_err() {
            return result.failure(token_result.err());
        }

        let mut parser = Parser::new(&token_result.ok().unwrap());
        let ast = parser.parse();

        if ast.error.is_some() {
            return result.failure(ast.error);
        }

        let mut interpreter = Interpreter::new(
            if self.precached_std_lib.is_some() {
                self.precached_std_lib.clone()
            } else {
                None
            },
            self.imported_modules.clone(),
        );
        let module_context = Rc::new(RefCell::new(Context::new(
            "<module>".to_string(),
            None,
            None,
            Some(interpreter.global_symbol_table.clone()),
        )));

        if let Some(std_lib) = self.precached_std_lib.clone() {
            let symbols: Vec<(String, Option<Rc<RefCell<Value>>>)> = std_lib
                .borrow()
                .symbols
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();

            for (name, value) in symbols {
                module_context
                    .borrow_mut()
                    .symbol_table
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .set(name, value);
            }
        }

        let module_result = interpreter.visit(ast.node.unwrap().as_ref(), module_context.clone());

        if module_result.error.is_some() {
            return result.failure(module_result.error);
        }

        self.imported_modules.borrow_mut().insert(
            file_to_import.clone(),
            module_context
                .borrow()
                .symbol_table
                .as_ref()
                .unwrap()
                .clone(),
        );

        let symbols: Vec<(String, Option<Rc<RefCell<Value>>>)> = module_context
            .borrow()
            .symbol_table
            .as_ref()
            .unwrap()
            .borrow()
            .symbols
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        for (name, value) in symbols {
            context
                .borrow_mut()
                .symbol_table
                .as_ref()
                .unwrap()
                .borrow_mut()
                .set(name, value);
        }

        result.success(Some(Number::null_value()))
    }

    fn visit_function_definition_node(
        &mut self,
        node: &FunctionDefinitionNode,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();

        let func_name = if node.var_name_token.is_some() {
            node.var_name_token
                .as_ref()
                .unwrap()
                .value
                .as_ref()
                .unwrap()
                .clone()
        } else {
            "".to_string()
        };
        let body_node = node.body_node.clone();
        let mut arg_names: Vec<String> = Vec::new();

        for arg_name in node.arg_name_tokens.iter() {
            arg_names.push(arg_name.value.as_ref().unwrap().clone());
        }

        let func_value = Rc::new(RefCell::new(Value::FunctionValue(Function::new(
            func_name.clone(),
            body_node,
            &arg_names,
            node.should_auto_return,
        ))));
        func_value.borrow_mut().set_context(Some(context.clone()));
        func_value
            .borrow_mut()
            .set_position(node.pos_start.clone(), node.pos_end.clone());

        if !&func_name.is_empty() {
            context
                .borrow_mut()
                .symbol_table
                .as_mut()
                .unwrap()
                .borrow_mut()
                .set(func_name, Some(func_value.clone()));
        }

        result.success(Some(func_value))
    }

    fn visit_call_node(&mut self, node: &CallNode, context: Rc<RefCell<Context>>) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let mut args: Vec<Rc<RefCell<Value>>> = Vec::new();

        let mut value_to_call =
            result.register(self.visit(node.node_to_call.as_ref(), context.clone()));

        if result.should_return() {
            return result;
        }

        // prevent recursion issues by borrowing already borrowed objects
        if let Some(v) = &mut value_to_call.as_mut().unwrap().try_borrow_mut().ok() {
            v.set_position(node.pos_start.clone(), node.pos_end.clone());
        }

        for arg_node in &node.arg_nodes {
            let arg = result.register(self.visit(arg_node.as_ref(), context.clone()));

            if result.should_return() {
                return result;
            }

            let arg = arg.unwrap();

            args.push(arg);
        }

        let mut return_value = result.register(match *value_to_call.unwrap().borrow() {
            Value::FunctionValue(ref value) => value.execute(&args, self),
            Value::BuiltInFunction(ref value) => value.execute(&args),
            _ => {
                return result.failure(Some(StandardError::new(
                    "expected function as call",
                    node.pos_start.as_ref().unwrap().clone(),
                    node.pos_end.as_ref().unwrap().clone(),
                    None,
                )));
            }
        });

        if result.should_return() {
            // if the call contains an error from 'uhoh', propagate it upward
            if result.should_propagate() {
                let err = result.error.as_mut().unwrap();
                err.pos_start = node.pos_start.as_ref().unwrap().clone();
                err.pos_end = node.pos_end.as_ref().unwrap().clone();

                println!("{err}");
            }

            return result;
        }

        return_value
            .as_mut()
            .unwrap()
            .borrow_mut()
            .set_position(node.pos_start.clone(), node.pos_end.clone());
        return_value
            .as_mut()
            .unwrap()
            .borrow_mut()
            .set_context(Some(context.clone()));

        result.success(Some(return_value.unwrap()))
    }

    fn visit_binary_operator_node(
        &mut self,
        node: &BinaryOperatorNode,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();

        let left = result.register(self.visit(node.left_node.as_ref(), context.clone()));

        if result.should_return() {
            return result;
        }

        let left = left.unwrap();

        let right = result.register(self.visit(node.right_node.as_ref(), context.clone()));

        if result.should_return() {
            return result;
        }

        let right = right.unwrap();

        let op = match node.op_token.token_type {
            TokenType::TT_PLUS => Some("+"),
            TokenType::TT_MINUS => Some("-"),
            TokenType::TT_MUL => Some("*"),
            TokenType::TT_DIV => Some("/"),
            TokenType::TT_POW => Some("^"),
            TokenType::TT_MOD => Some("%"),
            TokenType::TT_GT => Some(">"),
            TokenType::TT_LT => Some("<"),
            TokenType::TT_EE => Some("=="),
            TokenType::TT_NE => Some("!="),
            TokenType::TT_LTE => Some("<="),
            TokenType::TT_GTE => Some(">="),
            _ if node.op_token.matches(TokenType::TT_KEYWORD, "and") => Some("and"),
            _ if node.op_token.matches(TokenType::TT_KEYWORD, "or") => Some("or"),
            _ => None,
        };

        let operation_result = {
            let left_copy = left.borrow().clone();
            let mut left_borrow = left.borrow_mut();

            if Rc::ptr_eq(&left, &right) {
                // if we are comparing two of the same values, perform operation on a clone of itself
                left_borrow.perform_operation(op.unwrap_or(""), Rc::new(RefCell::new(left_copy)))
            } else if let Some(op) = op {
                left_borrow.perform_operation(op, right)
            } else {
                left_borrow.perform_operation("", right)
            }
        };

        match operation_result {
            Ok(val) => {
                val.borrow_mut()
                    .set_position(node.pos_start.clone(), node.pos_end.clone());
                result.success(Some(val))
            }
            Err(err) => result.failure(Some(err)),
        }
    }

    fn visit_unary_operator_node(
        &mut self,
        node: &UnaryOperatorNode,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let value = result.register(self.visit(node.node.as_ref(), context));

        if result.should_return() {
            return result;
        }

        let value = value.unwrap();

        let mut operation_result: Result<Rc<RefCell<Value>>, StandardError>;

        if node.op_token.token_type == TokenType::TT_MINUS {
            operation_result = value
                .borrow_mut()
                .perform_operation("*", Number::from(-1.0));
        } else if node.op_token.matches(TokenType::TT_KEYWORD, "not") {
            operation_result = value
                .borrow_mut()
                .perform_operation("not", Number::false_value());
        } else {
            operation_result = Err(StandardError::new(
                "unsupported unary operation",
                value.borrow().position_start().unwrap(),
                value.borrow().position_end().unwrap(),
                None,
            ))
        }

        if operation_result.is_err() {
            result.failure(operation_result.err())
        } else if operation_result.is_ok() {
            operation_result
                .as_mut()
                .ok()
                .unwrap()
                .borrow_mut()
                .set_position(node.pos_start.clone(), node.pos_end.clone());

            result.success(Some(operation_result.ok().unwrap()))
        } else {
            result.success(Some(Number::null_value()))
        }
    }

    fn visit_return_node(
        &mut self,
        node: &ReturnNode,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let mut value: Option<Rc<RefCell<Value>>> = None;

        if node.node_to_return.is_some() {
            value = result
                .register(self.visit(node.node_to_return.as_ref().unwrap().as_ref(), context));

            if result.should_return() {
                return result;
            }
        } else {
            value = Some(Number::null_value())
        }

        let value = value.unwrap();

        result.success_return(Some(value))
    }

    fn visit_continue_node(
        &mut self,
        node: &ContinueNode,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        RuntimeResult::new().success_continue()
    }

    fn visit_break_node(
        &mut self,
        node: &BreakNode,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        RuntimeResult::new().success_break()
    }
}
