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
use glang_parser::Parser;
use glang_parser::nodes::{
    ast_node::AstNode, binary_operator_node::BinaryOperatorNode, break_node::BreakNode,
    call_node::CallNode, const_assign_node::ConstAssignNode, continue_node::ContinueNode,
    for_node::ForNode, function_definition_node::FunctionDefinitionNode, if_node::IfNode,
    import_node::ImportNode, list_node::ListNode, number_node::NumberNode, return_node::ReturnNode,
    string_node::StringNode, try_except_node::TryExceptNode,
    unary_operator_node::UnaryOperatorNode, variable_access_node::VariableAccessNode,
    variable_assign_node::VariableAssignNode, while_node::WhileNode,
};
use std::{cell::RefCell, fs, path::Path, rc::Rc};

pub struct Interpreter {
    pub global_symbol_table: Rc<RefCell<SymbolTable>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let interpreter = Self {
            global_symbol_table: Rc::new(RefCell::new(SymbolTable::new(None))),
        };

        let builtins = [
            "bark", "chew", "dig", "bury", "tostring", "tonumber", "length", "uhoh", "type", "run",
            "_env",
        ];

        for builtin in &builtins {
            interpreter.global_symbol_table.borrow_mut().set(
                builtin.to_string(),
                Some(Value::BuiltInFunction(BuiltInFunction::new(builtin))),
            );
        }

        interpreter
    }

    pub fn evaluate(&mut self, src: &str, context: Rc<RefCell<Context>>) -> Option<StandardError> {
        let mut lexer = Lexer::new("<eval>", src.to_string());
        let token_result = lexer.make_tokens();

        if token_result.is_err() {
            return token_result.err();
        }

        let mut parser = Parser::new(&token_result.ok().unwrap());
        let ast = parser.parse();

        if ast.error.is_some() {
            return ast.error;
        }

        let result = self.visit(ast.node.unwrap(), context);
        result.error
    }

    pub fn visit(&mut self, node: Box<AstNode>, context: Rc<RefCell<Context>>) -> RuntimeResult {
        match node.as_ref() {
            AstNode::List(node) => self.visit_list_node(node, context),
            AstNode::Number(node) => self.visit_number_node(node, context),
            AstNode::Strings(node) => self.visit_string_node(node, context),
            AstNode::VariableAssign(node) => self.visit_variable_assign_node(node, context),
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
        let value: f64 = node.token.value.as_ref().unwrap().parse().unwrap();

        RuntimeResult::new().success(Some(
            Value::NumberValue(Number::new(value))
                .set_context(Some(context.clone()))
                .set_position(node.pos_start.clone(), node.pos_end.clone()),
        ))
    }

    fn visit_list_node(&mut self, node: &ListNode, context: Rc<RefCell<Context>>) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let mut elements: Vec<Value> = Vec::new();

        for element in node.element_nodes.iter() {
            let element_result = result.register(self.visit(element.to_owned(), context.clone()));

            if result.should_return() {
                return result;
            }

            elements.push(element_result.unwrap());
        }

        result.success(Some(
            Value::ListValue(List::new(elements))
                .set_context(Some(context.clone()))
                .set_position(node.pos_start.clone(), node.pos_end.clone()),
        ))
    }

    fn visit_string_node(
        &mut self,
        node: &StringNode,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        RuntimeResult::new().success(Some(
            Value::StringValue(Str::new(node.token.value.as_ref().unwrap().clone()))
                .set_context(Some(context.clone()))
                .set_position(node.pos_start.clone(), node.pos_end.clone()),
        ))
    }

    fn visit_variable_assign_node(
        &mut self,
        node: &VariableAssignNode,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let var_name = node.var_name_token.value.as_ref().unwrap().clone();
        let value = result.register(self.visit(node.value_node.clone(), context.clone()));

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
        let value = result.register(self.visit(node.value_node.clone(), context.clone()));

        if result.should_return() {
            return result;
        }

        if context
            .borrow()
            .symbol_table
            .as_ref()
            .unwrap()
            .borrow()
            .get(&const_name)
            .is_some()
        {
            return result.failure(Some(StandardError::new(
                "cannot reassign the value of a constant",
                node.pos_start.as_ref().unwrap().to_owned(),
                node.pos_end.as_ref().unwrap().to_owned(),
                None,
            )));
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
            .borrow_mut()
            .get(var_name.as_str())
            .clone();

        if value.is_none() {
            return result.failure(Some(StandardError::new(
                format!("variable name '{var_name}' is undefined").as_str(),
                node.pos_start.as_ref().unwrap().clone(),
                node.pos_end.as_ref().unwrap().clone(),
                None,
            )));
        }

        value = Some(
            value
                .clone()
                .unwrap()
                .set_position(node.pos_start.clone(), node.pos_end.clone())
                .set_context(Some(context.clone())),
        );

        result.success(value)
    }

    fn visit_if_node(&mut self, node: &IfNode, context: Rc<RefCell<Context>>) -> RuntimeResult {
        let mut result = RuntimeResult::new();

        for (condition, expr, should_return_null) in node.cases.iter() {
            let condition_value = result.register(self.visit(condition.clone(), context.clone()));

            if result.should_return() {
                return result;
            }

            let condition_value = condition_value.unwrap();

            if condition_value.is_true() {
                let expr_value = result.register(self.visit(expr.clone(), context.clone()));

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
            let else_value = result.register(self.visit(expr.clone(), context.clone()));

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

        let start_value = match result
            .register(self.visit(node.start_value_node.clone(), context.clone()))
            .unwrap()
        {
            Value::NumberValue(value) => Number::new(value.value),
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

        let end_value = match result
            .register(self.visit(node.end_value_node.clone(), context.clone()))
            .unwrap()
        {
            Value::NumberValue(value) => Number::new(value.value),
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

        if node.step_value_node.is_some() {
            step_value = match result
                .register(self.visit(
                    node.step_value_node.as_ref().unwrap().clone(),
                    context.clone(),
                ))
                .unwrap()
            {
                Value::NumberValue(value) => Number::new(value.value),
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

        let mut i = start_value.value;

        if step_value.value >= 0.0 {
            while i < end_value.value {
                context
                    .borrow_mut()
                    .symbol_table
                    .as_mut()
                    .unwrap()
                    .borrow_mut()
                    .set(
                        node.var_name_token.value.as_ref().unwrap().clone(),
                        Some(Value::NumberValue(Number::new(i))),
                    );
                i += step_value.value;

                let _ = result.register(self.visit(node.body_node.clone(), context.clone()));

                if result.should_return()
                    && !result.loop_should_continue
                    && !result.loop_should_break
                {
                    return result;
                }

                if result.loop_should_continue {
                    continue;
                }

                if result.loop_should_break {
                    break;
                }
            }
        } else {
            while i > end_value.value {
                context
                    .borrow_mut()
                    .symbol_table
                    .as_mut()
                    .unwrap()
                    .borrow_mut()
                    .set(
                        node.var_name_token.value.as_ref().unwrap().clone(),
                        Some(Value::NumberValue(Number::new(i))),
                    );
                i += step_value.value;

                let _ = result.register(self.visit(node.body_node.clone(), context.clone()));

                if result.should_return()
                    && !result.loop_should_continue
                    && !result.loop_should_break
                {
                    return result;
                }

                if result.loop_should_continue {
                    continue;
                }

                if result.loop_should_break {
                    break;
                }
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
                result.register(self.visit(node.condition_node.clone(), context.clone()));

            if result.should_return() {
                return result;
            }

            let condition = condition.unwrap();

            if !condition.is_true() {
                break;
            }

            let _ = result.register(self.visit(node.body_node.clone(), context.clone()));

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

        let _ = result.register(self.visit(node.try_body_node.clone(), context.clone()));
        let try_error = result.error.clone();

        if try_error.is_some() {
            context
                .borrow_mut()
                .symbol_table
                .as_mut()
                .unwrap()
                .borrow_mut()
                .set(
                    node.error_name_token.value.to_owned().unwrap(),
                    Some(Str::from(&try_error.unwrap().text)),
                );

            let _ = result.register(self.visit(node.except_body_node.clone(), context));

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
        let import = result.register(self.visit(node.node_to_import.to_owned(), context.clone()));

        if result.should_return() {
            return result;
        }

        let import_value = import.unwrap();
        let import_position_path = import_value.position_start().unwrap().filename.clone();

        let importing_dir = Path::new(&import_position_path)
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_path_buf();

        let file_to_import = match import_value {
            Value::StringValue(ref string) => string.as_string(),
            _ => {
                return result.failure(Some(StandardError::new(
                    "expected type string",
                    import_value.position_start().unwrap(),
                    import_value.position_end().unwrap(),
                    Some("add the '.glang' file you would like to import"),
                )));
            }
        };

        let file_to_import = importing_dir
            .join(&file_to_import)
            .to_string_lossy()
            .into_owned();

        if fs::exists(&file_to_import).is_err() || !&file_to_import.ends_with(".glang") {
            return result.failure(Some(StandardError::new(
                "file doesn't exist or isn't valid",
                import_value.position_start().unwrap(),
                import_value.position_end().unwrap(),
                Some("add the '.glang' file you would like to import"),
            )));
        }

        if file_to_import == import_position_path {
            return result.failure(Some(StandardError::new(
                "circular import",
                import_value.position_start().unwrap(),
                import_value.position_end().unwrap(),
                None,
            )));
        }

        let mut contents = String::new();

        match fs::read_to_string(&file_to_import) {
            Ok(extra) => contents.push_str(&extra),
            Err(_) => {
                return result.failure(Some(StandardError::new(
                    &format!("file contents couldn't be read properly on {file_to_import}"),
                    import_value.position_start().unwrap(),
                    import_value.position_end().unwrap(),
                    Some("add a UTF-8 encoded '.glang' file you would like to import"),
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

        let mut interpreter = Interpreter::new();
        let module_context = Rc::new(RefCell::new(Context::new(
            "<module>".to_string(),
            None,
            None,
        )));
        module_context.borrow_mut().symbol_table = Some(self.global_symbol_table.clone());
        let module_result = interpreter.visit(ast.node.unwrap(), module_context.clone());

        if module_result.error.is_some() {
            return result.failure(module_result.error);
        }

        let symbols: Vec<(String, Option<Value>)> = module_context
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

        let func_value = Value::FunctionValue(Function::new(
            func_name.clone(),
            body_node,
            &arg_names,
            node.should_auto_return,
        ))
        .set_context(Some(context.clone()))
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
        let mut args: Vec<Value> = Vec::new();

        let value_to_call = result.register(self.visit(node.node_to_call.clone(), context.clone()));

        if result.should_return() {
            return result;
        }

        let value_to_call = value_to_call
            .unwrap()
            .set_position(node.pos_start.clone(), node.pos_end.clone());

        for arg_node in &node.arg_nodes {
            let arg = result.register(self.visit(arg_node.to_owned(), context.clone()));

            if result.should_return() {
                return result;
            }

            let arg = arg.unwrap();

            args.push(arg);
        }

        let return_value = result.register(match value_to_call {
            Value::FunctionValue(value) => value.execute(&args),
            Value::BuiltInFunction(value) => value.execute(&args),
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
            return result;
        }

        let return_value = return_value
            .unwrap()
            .set_position(node.pos_start.clone(), node.pos_end.clone())
            .set_context(Some(context.clone()));

        result.success(Some(return_value))
    }

    fn visit_binary_operator_node(
        &mut self,
        node: &BinaryOperatorNode,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let left = result.register(self.visit(node.left_node.clone(), context.clone()));

        if result.should_return() {
            return result;
        }

        let mut left = left.unwrap();

        let right = result.register(self.visit(node.right_node.clone(), context.clone()));

        if result.should_return() {
            return result;
        }

        let right = right.unwrap();

        let operation_result: Result<Value, StandardError>;

        if node.op_token.token_type == TokenType::TT_PLUS {
            operation_result = left.perform_operation("+", right);
        } else if node.op_token.token_type == TokenType::TT_MINUS {
            operation_result = left.perform_operation("-", right);
        } else if node.op_token.token_type == TokenType::TT_MUL {
            operation_result = left.perform_operation("*", right);
        } else if node.op_token.token_type == TokenType::TT_DIV {
            operation_result = left.perform_operation("/", right);
        } else if node.op_token.token_type == TokenType::TT_POW {
            operation_result = left.perform_operation("^", right);
        } else if node.op_token.token_type == TokenType::TT_MOD {
            operation_result = left.perform_operation("%", right);
        } else if node.op_token.token_type == TokenType::TT_GT {
            operation_result = left.perform_operation(">", right);
        } else if node.op_token.token_type == TokenType::TT_LT {
            operation_result = left.perform_operation("<", right);
        } else if node.op_token.token_type == TokenType::TT_EE {
            operation_result = left.perform_operation("==", right);
        } else if node.op_token.token_type == TokenType::TT_NE {
            operation_result = left.perform_operation("!=", right);
        } else if node.op_token.token_type == TokenType::TT_LTE {
            operation_result = left.perform_operation("<=", right);
        } else if node.op_token.token_type == TokenType::TT_GTE {
            operation_result = left.perform_operation(">=", right);
        } else if node.op_token.matches(TokenType::TT_KEYWORD, "and") {
            operation_result = left.perform_operation("and", right);
        } else if node.op_token.matches(TokenType::TT_KEYWORD, "or") {
            operation_result = left.perform_operation("or", right);
        } else {
            operation_result = left.perform_operation("", right);
        }

        if operation_result.is_err() {
            result.failure(operation_result.err())
        } else if operation_result.is_ok() {
            return result.success(Some(
                operation_result
                    .ok()
                    .unwrap()
                    .set_position(node.pos_start.clone(), node.pos_end.clone()),
            ));
        } else {
            return result.success(Some(Number::null_value()));
        }
    }

    fn visit_unary_operator_node(
        &mut self,
        node: &UnaryOperatorNode,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let value = result.register(self.visit(node.node.clone(), context));

        if result.should_return() {
            return result;
        }

        let mut value = value.unwrap();

        let operation_result: Result<Value, StandardError>;

        if node.op_token.token_type == TokenType::TT_MINUS {
            operation_result = value.perform_operation("*", Number::from(-1.0));
        } else if node.op_token.matches(TokenType::TT_KEYWORD, "not") {
            operation_result = value.perform_operation("not", Number::false_value());
        } else {
            operation_result = Err(StandardError::new(
                "unsupported unary operation",
                value.position_start().unwrap(),
                value.position_end().unwrap(),
                None,
            ))
        }

        if operation_result.is_err() {
            result.failure(operation_result.err())
        } else if operation_result.is_ok() {
            return result.success(Some(
                operation_result
                    .ok()
                    .unwrap()
                    .set_position(node.pos_start.clone(), node.pos_end.clone()),
            ));
        } else {
            return result.success(Some(Number::null_value()));
        }
    }

    fn visit_return_node(
        &mut self,
        node: &ReturnNode,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let mut value: Option<Value> = None;

        if node.node_to_return.is_some() {
            value =
                result.register(self.visit(node.node_to_return.as_ref().unwrap().clone(), context));

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
