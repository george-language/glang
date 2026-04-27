use crate::{
    BuiltInFunction, Context, Function, List, Number, RuntimeResult, Str, SymbolTable, Value,
};
use glang_attributes::{BUILT_IN_FUNCTIONS, StandardError};
use glang_lexer::{Lexer, lex};
use glang_parser::{
    AstArena, AstNode, BinaryOperatorNode, CallNode, ConstAssignNode, ForEachNode, ForNode,
    FunctionDefinitionNode, IfNode, ImportNode, ListNode, NodeID, NumberNode, Parser, ReturnNode,
    StringNode, TryExceptNode, UnaryOperatorNode, VariableAccessNode, VariableAssignNode,
    VariableRessignNode, WhileNode, parse,
};
use glang_tooling::get_latest_version;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    rc::Rc,
    time::Instant,
};

pub fn interpret(ast: AstArena, contents: &str) -> Option<StandardError> {
    let interpreting_time = Instant::now();

    let mut interpreter = Interpreter::new(ast.clone(), contents);
    let context = Rc::new(RefCell::new(Context::new(
        None,
        None,
        interpreter.global_symbol_table.clone(),
    )));

    if !cfg!(feature = "no-std") {
        interpreter.preload_standard_library(context.clone());
    }

    let registry = glang_tooling::read_registry();

    for (name, info) in registry.packages {
        let version = info
            .get(&get_latest_version(&name).unwrap().to_string())
            .unwrap();
        let entry = version.get("entry").unwrap();

        interpreter
            .global_symbol_table
            .borrow_mut()
            .set(name.to_owned(), Str::from(&entry));
    }

    let result = interpreter.visit(
        NodeID(ast.nodes.len() - 1),
        &interpreter.arena.clone(),
        context.clone(),
    );

    if cfg!(feature = "benchmark") {
        println!(
            "Time to interpret: {:?}ms",
            interpreting_time.elapsed().as_millis()
        );
    }

    if result.should_propagate() {
        // if the error is propagating, it is already displayed in the terminal
        None
    } else {
        result.error
    }
}

pub struct Interpreter {
    pub global_symbol_table: Rc<RefCell<SymbolTable>>,
    pub cached_standard_library: Option<Rc<RefCell<SymbolTable>>>,
    pub arena: Rc<AstArena>,
    cached_modules: Rc<RefCell<HashMap<PathBuf, Rc<RefCell<SymbolTable>>>>>,
    contents: String,
}

impl Interpreter {
    pub fn new(arena: AstArena, contents: &str) -> Self {
        let interpreter = Self {
            global_symbol_table: Rc::new(RefCell::new(SymbolTable::new(None))),
            cached_standard_library: None,
            cached_modules: Rc::new(RefCell::new(HashMap::new())),
            arena: Rc::new(arena),
            contents: contents.to_owned(),
        };

        for builtin in BUILT_IN_FUNCTIONS {
            interpreter
                .global_symbol_table
                .borrow_mut()
                .set(builtin.to_string(), BuiltInFunction::from(builtin));
        }

        interpreter
    }

    pub fn preload_standard_library(&mut self, context: Rc<RefCell<Context>>) {
        if let Some(e) = self.evaluate(
            "fetch _env(\"GLANG_STD\") + \"/core/lib.glang\";",
            context.clone(),
        ) {
            println!("{}", e);

            return;
        }

        self.cached_standard_library = Some(context.borrow().symbol_table.clone());
    }

    pub fn evaluate(&mut self, src: &str, context: Rc<RefCell<Context>>) -> Option<StandardError> {
        let mut lexer = Lexer::new(Path::new("<eval>"), src);
        let token_result = lexer.make_tokens();

        if token_result.is_err() {
            return token_result.err();
        }

        let mut parser = Parser::new(&token_result.ok().unwrap(), lexer.contents());
        let ast = parser.parse();

        if ast.error.is_some() {
            return ast.error;
        }

        let result = self.visit(ast.node, &parser.arena, context);
        result.error
    }

    pub fn visit(
        &mut self,
        node: NodeID,
        arena: &AstArena,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let node = arena.get(node);

        match node {
            AstNode::List(node) => self.visit_list_node(node, arena, context),
            AstNode::Number(node) => self.visit_number_node(node, context),
            AstNode::Strings(node) => self.visit_string_node(node, context),
            AstNode::VariableAssign(node) => self.visit_variable_assign_node(node, arena, context),
            AstNode::VariableReassign(node) => {
                self.visit_variable_reassign_node(node, arena, context)
            }
            AstNode::ConstAssign(node) => self.visit_const_assign_node(node, arena, context),
            AstNode::VariableAccess(node) => self.visit_variable_access_node(node, context),
            AstNode::If(node) => self.visit_if_node(node, arena, context),
            AstNode::Import(node) => self.visit_import_node(node, arena, context),
            AstNode::For(node) => self.visit_for_node(node, arena, context),
            AstNode::ForEach(node) => self.visit_for_each_node(node, arena, context),
            AstNode::While(node) => self.visit_while_node(node, arena, context),
            AstNode::TryExcept(node) => self.visit_try_except_node(node, arena, context),
            AstNode::FunctionDefinition(node) => {
                self.visit_function_definition_node(node, arena, context)
            }
            AstNode::Call(node) => self.visit_call_node(node, arena, context),
            AstNode::BinaryOperator(node) => self.visit_binary_operator_node(node, arena, context),
            AstNode::UnaryOperator(node) => self.visit_unary_operator_node(node, arena, context),
            AstNode::Return(node) => self.visit_return_node(node, arena, context),
            AstNode::Continue(_) => self.visit_continue_node(),
            AstNode::Break(_) => self.visit_break_node(),
        }
    }

    fn visit_number_node(&self, node: &NumberNode, context: Rc<RefCell<Context>>) -> RuntimeResult {
        let value = Number::from(node.value);
        value.borrow_mut().set_context(Some(context.clone()));
        value.borrow_mut().set_span(node.span.clone());

        RuntimeResult::new().success(value)
    }

    fn visit_list_node(
        &mut self,
        node: &ListNode,
        arena: &AstArena,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let mut elements: Vec<Rc<RefCell<Value>>> = Vec::new();

        for element in node.element_nodes.iter() {
            let element_result =
                result.register(self.visit(element.to_owned(), &arena, context.clone()));

            if result.should_return() {
                return result;
            }

            elements.push(element_result);
        }

        let list = List::from(elements);
        list.borrow_mut().set_context(Some(context.clone()));
        list.borrow_mut().set_span(node.span.clone());

        result.success(list)
    }

    fn visit_string_node(
        &mut self,
        node: &StringNode,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let string = Str::from(&node.value);
        string.borrow_mut().set_context(Some(context.clone()));
        string.borrow_mut().set_span(node.span.clone());

        RuntimeResult::new().success(string)
    }

    fn visit_variable_assign_node(
        &mut self,
        node: &VariableAssignNode,
        arena: &AstArena,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let var_name = node.name.clone();

        if self.is_constant(&var_name, context.clone()) {
            return result.failure(StandardError::new(
                "cannot reassign the value of a constant",
                node.span.clone(),
                None,
            ));
        }

        let value = result.register(self.visit(node.value_node, &arena, context.clone()));

        if result.should_return() {
            return result;
        }

        context
            .borrow_mut()
            .symbol_table
            .borrow_mut()
            .set(var_name, value.clone());

        result.success(value)
    }

    fn visit_variable_reassign_node(
        &mut self,
        node: &VariableRessignNode,
        arena: &AstArena,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let var_name = node.name.clone();

        if self.is_constant(&var_name, context.clone()) {
            return result.failure(StandardError::new(
                "cannot reassign the value of a constant",
                node.span.clone(),
                None,
            ));
        }

        if context
            .borrow()
            .symbol_table
            .borrow()
            .get(&var_name)
            .is_none()
        {
            return result.failure(StandardError::new(
                format!("variable name '{var_name}' is undefined").as_str(),
                node.span.clone(),
                Some("define a variable with the syntax 'obj <variable name> = <value>;'"),
            ));
        }

        let value = result.register(self.visit(node.value_node, &arena, context.clone()));

        if result.should_return() {
            return result;
        }

        context
            .borrow_mut()
            .symbol_table
            .borrow_mut()
            .set(var_name, value.clone());

        result.success(value)
    }

    fn visit_const_assign_node(
        &mut self,
        node: &ConstAssignNode,
        arena: &AstArena,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let const_name = node.name.clone();

        if self.is_constant(&const_name, context.clone()) {
            return result.failure(StandardError::new(
                "cannot reassign the value of a constant",
                node.span.clone(),
                None,
            ));
        }

        let value = result.register(self.visit(node.value_node, &arena, context.clone()));
        let mut copied_value: Option<Rc<RefCell<Value>>> = None;

        if result.should_return() {
            return result;
        }

        // if the value we are accessing is not a constant, we copy that value in place
        if !value.borrow().is_const() {
            copied_value = Some(Rc::new(RefCell::new(value.borrow().clone())));
            copied_value.as_mut().unwrap().borrow_mut().set_const(true); // now a constant, cause we cloned
        }

        context.borrow_mut().symbol_table.borrow_mut().set(
            const_name,
            if let Some(cp) = copied_value {
                cp.clone()
            } else {
                value.clone()
            },
        );

        result.success(value)
    }

    fn visit_variable_access_node(
        &mut self,
        node: &VariableAccessNode,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let var_name = node.name.clone();
        let mut value = context
            .borrow()
            .symbol_table
            .borrow()
            .get(var_name.as_str())
            .clone();

        if value.is_none() {
            return result.failure(StandardError::new(
                format!("variable name '{var_name}' is undefined").as_str(),
                node.span.clone(),
                Some("define a variable with the syntax 'obj <variable name> = <value>;'"),
            ));
        }

        if let Some(mut v) = value.clone() {
            // if the value we are accessing is a constant, we copy the constant
            if v.borrow().is_const() {
                v = Rc::new(RefCell::new(value.as_ref().unwrap().borrow().clone()));
                v.borrow_mut().set_const(false); // no longer a constant, cause we cloned
            }

            // prevent recursion issues by borrowing already borrowed objects
            if let Ok(v) = &mut value.as_mut().unwrap().try_borrow_mut() {
                v.set_context(Some(context.clone()));
                v.set_span(node.span.clone());
            }

            return result.success(v);
        } else {
            return result.failure(StandardError::new(
                format!("variable name '{var_name}' is undefined").as_str(),
                node.span.clone(),
                Some("define a variable with the syntax 'obj <variable name> = <value>;'"),
            ));
        }
    }

    fn visit_if_node(
        &mut self,
        node: &IfNode,
        arena: &AstArena,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();

        for (condition, expr, should_return_null) in node.cases.iter() {
            let condition_value =
                result.register(self.visit(condition.to_owned(), &arena, context.clone()));

            if result.should_return() {
                return result;
            }

            if condition_value.borrow().is_true() {
                let expr_value =
                    result.register(self.visit(expr.to_owned(), &arena, context.clone()));

                if result.should_return() {
                    return result;
                }

                return result.success(if *should_return_null {
                    Number::null_value()
                } else {
                    expr_value
                });
            }
        }

        if node.else_case.is_some() {
            let (expr, should_return_null) = node.else_case.as_ref().unwrap().clone();
            let else_value = result.register(self.visit(expr, &arena, context.clone()));

            if result.should_return() {
                return result;
            }

            return result.success(if should_return_null {
                Number::null_value()
            } else {
                else_value
            });
        }

        result.success(Number::null_value())
    }

    fn visit_for_node(
        &mut self,
        node: &ForNode,
        arena: &AstArena,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();

        let start_value = match *result
            .register(self.visit(node.start_value_node, &arena, context.clone()))
            .borrow()
        {
            Value::NumberValue(ref value) => Number::new(value.value),
            _ => {
                return result.failure(StandardError::new(
                    "expected type number",
                    arena.span(node.start_value_node),
                    None,
                ));
            }
        };

        if result.should_return() {
            return result;
        }

        let end_value = match *result
            .register(self.visit(node.end_value_node, &arena, context.clone()))
            .borrow()
        {
            Value::NumberValue(ref value) => Number::new(value.value),
            _ => {
                return result.failure(StandardError::new(
                    "expected type number",
                    arena.span(node.end_value_node),
                    None,
                ));
            }
        };

        if result.should_return() {
            return result;
        }

        let step_value: Number;

        if let Some(step_value_node) = &node.step_value_node {
            step_value = match *result
                .register(self.visit(step_value_node.to_owned(), &arena, context.clone()))
                .borrow()
            {
                Value::NumberValue(ref value) => Number::new(value.value),
                _ => {
                    return result.failure(StandardError::new(
                        "expected type number",
                        start_value.span,
                        None,
                    ));
                }
            };

            if result.should_return() {
                return result;
            }
        } else {
            step_value = Number::new(1.0);
        }

        if step_value.value == 0.0 {
            return result.failure(StandardError::new(
                "step value of a 'walk' loop cannot be 0",
                arena.span(node.step_value_node.unwrap()),
                Some("use a step value like 'step = 1' to control how many iteration steps occur"),
            ));
        }

        let iterator_name = node.iterator_name.clone();
        let symbol_table = context.borrow().symbol_table.clone();

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
                .set(iterator_name.clone(), Number::from(i));

            let _ = result.register(self.visit(node.body_node, &arena, context.clone()));

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

        result.success(Number::null_value())
    }

    fn visit_for_each_node(
        &mut self,
        node: &ForEachNode,
        arena: &AstArena,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();

        let elements = match *result
            .register(self.visit(node.iterator_node, &arena, context.clone()))
            .borrow()
        {
            Value::ListValue(ref v) => v.elements.clone(),
            Value::StringValue(ref v) => {
                let mut elements = Vec::new();

                for char in v.value.chars() {
                    elements.push(Str::from(&char.to_string()));
                }

                elements
            }
            _ => {
                return result.failure(StandardError::new(
                    "object is not iterable",
                    arena.span(node.iterator_node),
                    None,
                ));
            }
        };

        if result.should_return() {
            return result;
        }

        let iterator_name = node.iterator_name.clone();
        let symbol_table = context.borrow().symbol_table.clone();

        for i in elements {
            symbol_table.borrow_mut().set(iterator_name.clone(), i);

            let _ = result.register(self.visit(node.body_node, &arena, context.clone()));

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

        result.success(Number::null_value())
    }

    fn visit_while_node(
        &mut self,
        node: &WhileNode,
        arena: &AstArena,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();

        loop {
            let condition =
                result.register(self.visit(node.condition_node, &arena, context.clone()));

            if result.should_return() {
                return result;
            }

            if !condition.borrow().is_true() {
                break;
            }

            let _ = result.register(self.visit(node.body_node, &arena, context.clone()));

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

        result.success(Number::null_value())
    }

    fn visit_try_except_node(
        &mut self,
        node: &TryExceptNode,
        arena: &AstArena,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();

        let _ = result.register(self.visit(node.try_body_node, &arena, context.clone()));
        let try_error = result.error.clone();

        if try_error.is_some() {
            let output_error = Str::from(&try_error.unwrap().text);
            output_error.borrow_mut().set_const(true);

            context
                .borrow_mut()
                .symbol_table
                .borrow_mut()
                .set(node.passed_error.clone(), output_error);

            let _ = result.register(self.visit(node.except_body_node, &arena, context));

            if result.error.is_some() {
                return result;
            }

            if result.should_return() {
                return result;
            }
        } else if result.should_return() {
            return result;
        }

        result.success(Number::null_value())
    }

    fn visit_import_node(
        &mut self,
        node: &ImportNode,
        arena: &AstArena,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();

        let import_value =
            result.register(self.visit(node.node_to_import, &arena, context.clone()));

        if result.should_return() {
            return result;
        }

        let importing_path = import_value.borrow().span().filename.clone();

        let importing_dir = Path::new(&importing_path)
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_path_buf();

        let file_to_import = importing_dir.join(PathBuf::from(match *import_value.borrow() {
            Value::StringValue(ref string) => string.value.clone(),
            _ => {
                return result.failure(StandardError::new(
                    "expected type string",
                    import_value.borrow().span(),
                    Some("add the '.glang' file to import"),
                ));
            }
        }));

        if !(file_to_import.exists() || !file_to_import.ends_with(".glang")) {
            return result.failure(StandardError::new(
                "invalid import",
                import_value.borrow().span(),
                Some("add the '.glang' file to import"),
            ));
        }

        if file_to_import == importing_path {
            return result.failure(StandardError::new(
                "circular import",
                import_value.borrow().span(),
                None,
            ));
        }

        // if we already have imported modules stored, then use cached ones
        if let Some(cached_symtab) = self.cached_modules.borrow().get(&file_to_import) {
            for (name, value) in cached_symtab.borrow().symbols.clone() {
                context
                    .borrow_mut()
                    .symbol_table
                    .borrow_mut()
                    .set(name, value);
            }

            return result.success(Number::null_value());
        }

        let mut contents = String::new();

        match fs::read_to_string(&file_to_import) {
            Ok(extra) => contents.push_str(&extra),
            Err(_) => {
                return result.failure(StandardError::new(
                    &format!(
                        "file contents couldn't be read properly on {}",
                        file_to_import.to_string_lossy()
                    ),
                    import_value.borrow().span(),
                    Some("add a UTF-8 encoded '.glang' file to import"),
                ));
            }
        }

        let ast_result = match lex(&file_to_import, &contents) {
            Ok(tokens) => match parse(&tokens, &contents) {
                Ok(ast) => Ok(ast),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        };

        let ast_node = match ast_result {
            Ok(ast_node) => ast_node,
            Err(e) => return result.failure(e),
        };

        let mut interpreter = Interpreter::new(ast_node.clone(), &contents);
        interpreter.cached_standard_library = self.cached_standard_library.clone();
        interpreter.cached_modules = self.cached_modules.clone();
        let module_context = Rc::new(RefCell::new(Context::new(
            None,
            None,
            interpreter.global_symbol_table.clone(),
        )));

        if let Some(std_lib) = self.cached_standard_library.clone() {
            for (name, value) in std_lib.borrow().symbols.clone() {
                module_context
                    .borrow_mut()
                    .symbol_table
                    .borrow_mut()
                    .set(name, value);
            }
        }

        let module_result = interpreter.visit(
            NodeID(ast_node.nodes.len() - 1),
            &interpreter.arena.clone(),
            module_context.clone(),
        );

        if let Some(e) = module_result.error {
            return result.failure(e);
        }

        self.cached_modules.borrow_mut().insert(
            file_to_import.clone(),
            module_context.borrow().symbol_table.clone(),
        );

        for (name, value) in module_context
            .borrow()
            .symbol_table
            .borrow()
            .symbols
            .clone()
        {
            context
                .borrow_mut()
                .symbol_table
                .borrow_mut()
                .set(name, value);
        }

        result.success(Number::null_value())
    }

    fn visit_function_definition_node(
        &mut self,
        node: &FunctionDefinitionNode,
        arena: &AstArena,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();

        let func_name = if let Some(ref name) = node.name {
            name.clone()
        } else {
            "".to_string()
        };
        let body_node = node.body_node.clone();
        let mut arg_names: Vec<String> = Vec::new();

        for arg_name_tok in node.argument_names.iter() {
            arg_names.push(arg_name_tok.value.clone().unwrap());
        }

        let mut seen = HashSet::new();

        for (i, name) in arg_names.iter().enumerate() {
            if !seen.insert(name) {
                return result.failure(StandardError::new(
                    "duplicate argument",
                    node.argument_names[i].span.clone(),
                    Some(format!("remove the duplicate argument '{}'", name).as_str()),
                ));
            }
        }

        let func_value = Rc::new(RefCell::new(Value::FunctionValue(Function::new(
            func_name.clone(),
            body_node,
            arena.to_owned(),
            &arg_names,
            node.should_auto_return,
        ))));
        func_value.borrow_mut().set_context(Some(context.clone()));
        func_value.borrow_mut().set_span(node.span.clone());

        if !&func_name.is_empty() {
            context
                .borrow_mut()
                .symbol_table
                .borrow_mut()
                .set(func_name, func_value.clone());
        }

        result.success(func_value)
    }

    fn visit_call_node(
        &mut self,
        node: &CallNode,
        arena: &AstArena,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let mut args: Vec<Rc<RefCell<Value>>> = Vec::new();

        let value_to_call = result.register(self.visit(node.node_to_call, &arena, context.clone()));

        if result.should_return() {
            return result;
        }

        // prevent recursion issues by borrowing already borrowed objects
        if let Some(v) = &mut value_to_call.try_borrow_mut().ok() {
            v.set_span(node.span.clone());
        }

        for arg_node in &node.arg_nodes {
            let arg = result.register(self.visit(arg_node.to_owned(), &arena, context.clone()));

            if result.should_return() {
                return result;
            }

            args.push(arg);
        }

        let return_value = result.register(match *value_to_call.borrow() {
            Value::FunctionValue(ref value) => value.execute(&args, self),
            Value::BuiltInFunction(ref value) => value.execute(&args),
            _ => {
                return result.failure(StandardError::new(
                    "object is not callable",
                    node.span.clone(),
                    None,
                ));
            }
        });

        if result.should_return() {
            // if the call contains an error from 'uhoh', propagate it upward
            if result.should_propagate() {
                let err = result.error.as_mut().unwrap();
                err.span = node.span.clone();

                if !err.span.filename.exists() {
                    err.contents = Some(self.contents.clone());
                }

                println!("{err}");
            }

            return result;
        }

        return_value.borrow_mut().set_span(node.span.clone());
        return_value.borrow_mut().set_context(Some(context.clone()));

        result.success(return_value)
    }

    fn visit_binary_operator_node(
        &mut self,
        node: &BinaryOperatorNode,
        arena: &AstArena,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();

        let left = result.register(self.visit(node.left_node, &arena, context.clone()));

        if result.should_return() {
            return result;
        }

        let right = result.register(self.visit(node.right_node, &arena, context.clone()));

        if result.should_return() {
            return result;
        }

        let operation_result = {
            let left_copy = left.borrow().clone();
            let mut left_borrow = left.borrow_mut();

            if Rc::ptr_eq(&left, &right) {
                // if we are comparing two of the same values, perform operation on a clone of itself
                left_borrow.perform_operation(&node.operator, Rc::new(RefCell::new(left_copy)))
            } else {
                left_borrow.perform_operation(&node.operator, right)
            }
        };

        match operation_result {
            Ok(val) => {
                val.borrow_mut().set_span(node.span.clone());
                result.success(val)
            }
            Err(err) => result.failure(err),
        }
    }

    fn visit_unary_operator_node(
        &mut self,
        node: &UnaryOperatorNode,
        arena: &AstArena,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let value = result.register(self.visit(node.node, &arena, context));

        if result.should_return() {
            return result;
        }

        let operation_result: Result<Rc<RefCell<Value>>, StandardError>;

        match node.operator.as_str() {
            "-1" => {
                operation_result = value
                    .borrow_mut()
                    .perform_operation("*", Number::from(-1.0));
            }
            "not" => {
                operation_result = value
                    .borrow_mut()
                    .perform_operation("not", Number::false_value());
            }
            _ => {
                operation_result = Err(StandardError::new(
                    "unsupported unary operation",
                    value.borrow().span(),
                    None,
                ))
            }
        }

        match operation_result {
            Ok(res) => {
                res.borrow_mut().set_span(node.span.clone());

                return result.success(res);
            }
            Err(e) => return result.failure(e),
        }
    }

    fn visit_return_node(
        &mut self,
        node: &ReturnNode,
        arena: &AstArena,
        context: Rc<RefCell<Context>>,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let value: Rc<RefCell<Value>>;

        if node.node_to_return.is_some() {
            value = result.register(self.visit(node.node_to_return.unwrap(), &arena, context));

            if result.should_return() {
                return result;
            }
        } else {
            value = Number::null_value()
        }

        result.success_return(value)
    }

    fn visit_continue_node(&mut self) -> RuntimeResult {
        RuntimeResult::new().success_continue()
    }

    fn visit_break_node(&mut self) -> RuntimeResult {
        RuntimeResult::new().success_break()
    }

    fn is_constant(&self, name: &str, context: Rc<RefCell<Context>>) -> bool {
        let constant = context.borrow().symbol_table.borrow().get(&name);

        if let Some(c) = constant {
            return c.borrow().is_const();
        }

        false
    }
}
