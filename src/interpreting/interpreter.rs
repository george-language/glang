use std::{cell::RefCell, rc::Rc};

use crate::{
    errors::standard_error::StandardError,
    interpreting::{context::Context, runtime_result::RuntimeResult, symbol_table::SymbolTable},
    lexing::token_type::TokenType,
    nodes::{
        ast_node::AstNode, binary_operator_node::BinaryOperatorNode, break_node::BreakNode,
        call_node::CallNode, continue_node::ContinueNode, for_node::ForNode,
        function_definition_node::FunctionDefinitionNode, if_node::IfNode, list_node::ListNode,
        number_node::NumberNode, return_node::ReturnNode, string_node::StringNode,
        unary_operator_node::UnaryOperatorNode, variable_access_node::VariableAccessNode,
        variable_assign_node::VariableAssignNode, while_node::WhileNode,
    },
    values::{
        built_in_function::BuiltInFunction, function::Function, list::List, number::Number,
        string::StringObj, value::Value,
    },
};

pub struct Interpreter {
    pub global_symbol_table: Rc<RefCell<SymbolTable>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let interpreter = Interpreter {
            global_symbol_table: Rc::new(RefCell::new(SymbolTable::new(None))),
        };

        let builtins = [
            "bark", "chew", "dig", "bury", "tostring", "tonumber", "length", "clear", "uhoh",
            "type", "fetch", "docs", "run",
        ];

        for builtin in &builtins {
            interpreter.global_symbol_table.borrow_mut().set(
                builtin.to_string(),
                Some(Box::new(Value::BuiltInFunction(BuiltInFunction::new(
                    builtin,
                    interpreter.global_symbol_table.clone(),
                )))),
            );
        }

        interpreter
    }

    pub fn visit(&mut self, node: Box<AstNode>, context: &mut Context) -> RuntimeResult {
        match node.as_ref() {
            AstNode::List(node) => {
                return self.visit_list_node(&node, context);
            }
            AstNode::Number(node) => {
                return self.visit_number_node(&node, context);
            }
            AstNode::Strings(node) => {
                return self.visit_string_node(&node, context);
            }
            AstNode::VariableAssign(node) => {
                return self.visit_variable_assign_node(&node, context);
            }
            AstNode::VariableAccess(node) => {
                return self.visit_variable_access_node(&node, context);
            }
            AstNode::If(node) => {
                return self.visit_if_node(&node, context);
            }
            AstNode::For(node) => {
                return self.visit_for_node(&node, context);
            }
            AstNode::While(node) => {
                return self.visit_while_node(&node, context);
            }
            AstNode::FunctionDefinition(node) => {
                return self.visit_function_definition_node(&node, context);
            }
            AstNode::Call(node) => {
                return self.visit_call_node(&node, context);
            }
            AstNode::BinaryOperator(node) => {
                return self.visit_binary_operator_node(&node, context);
            }
            AstNode::UnaryOperator(node) => {
                return self.visit_unary_operator_node(&node, context);
            }
            AstNode::Return(node) => {
                return self.visit_return_node(&node, context);
            }
            AstNode::Continue(node) => {
                return self.visit_continue_node(&node, context);
            }
            AstNode::Break(node) => {
                return self.visit_break_node(&node, context);
            }
            _ => {
                panic!(
                    "CRITICAL ERROR: NO METHOD DEFINED FOR NODE TYPE:\n {:#?}",
                    node
                );
            }
        }
    }

    pub fn visit_number_node(&self, node: &NumberNode, context: &mut Context) -> RuntimeResult {
        let value: f64 = node.token.value.as_ref().unwrap().parse().unwrap();

        RuntimeResult::new().success(Some(
            Value::NumberValue(Number::new(value))
                .set_context(Some(context.clone()))
                .set_position(node.pos_start.clone(), node.pos_end.clone()),
        ))
    }

    pub fn visit_list_node(&mut self, node: &ListNode, context: &mut Context) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let mut elements: Vec<Option<Box<Value>>> = Vec::new();

        for element in &node.element_nodes {
            elements.push(result.register(self.visit(element.as_ref().unwrap().clone(), context)));

            if result.should_return() {
                return result;
            }
        }

        result.success(Some(
            Value::ListValue(List::new(elements))
                .set_context(Some(context.clone()))
                .set_position(node.pos_start.clone(), node.pos_end.clone()),
        ))
    }

    pub fn visit_string_node(&mut self, node: &StringNode, context: &mut Context) -> RuntimeResult {
        RuntimeResult::new().success(Some(
            Value::StringValue(StringObj::new(node.token.value.as_ref().unwrap().clone()))
                .set_context(Some(context.clone()))
                .set_position(node.pos_start.clone(), node.pos_end.clone()),
        ))
    }

    pub fn visit_variable_assign_node(
        &mut self,
        node: &VariableAssignNode,
        context: &mut Context,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let var_name = node.var_name_token.value.as_ref().unwrap().clone();
        let value = result.register(self.visit(node.value_node.clone(), context));

        if result.should_return() {
            return result;
        }

        context
            .symbol_table
            .as_mut()
            .unwrap()
            .borrow_mut()
            .set(var_name, value.clone());

        result.success(value)
    }

    pub fn visit_variable_access_node(
        &mut self,
        node: &VariableAccessNode,
        context: &mut Context,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let var_name = node.var_name_token.value.as_ref().unwrap();
        let mut value = context
            .symbol_table
            .as_ref()
            .unwrap()
            .borrow_mut()
            .get(var_name.as_str())
            .clone();

        if value.is_none() {
            return result.failure(Some(StandardError::new(
                format!("variable name '{}' is undefined", var_name).as_str(),
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

    pub fn visit_if_node(&mut self, node: &IfNode, context: &mut Context) -> RuntimeResult {
        let mut result = RuntimeResult::new();

        for (condition, expr, should_return_null) in &node.cases {
            let condition_value = result.register(self.visit(condition.clone(), context));

            if result.should_return() {
                return result;
            }

            let condition_value = condition_value.unwrap();

            if condition_value.is_true() {
                let expr_value = result.register(self.visit(expr.clone(), context));

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
            let else_value = result.register(self.visit(expr.clone(), context));

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

    pub fn visit_for_node(&mut self, node: &ForNode, context: &mut Context) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let mut elements: Vec<Option<Box<Value>>> = Vec::new();

        let start_value = match result
            .register(self.visit(node.start_value_node.clone(), context))
            .unwrap()
            .as_ref()
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
            .register(self.visit(node.end_value_node.clone(), context))
            .unwrap()
            .as_ref()
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
                .register(self.visit(node.step_value_node.as_ref().unwrap().clone(), context))
                .unwrap()
                .as_ref()
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
                context.symbol_table.as_mut().unwrap().borrow_mut().set(
                    node.var_name_token.value.as_ref().unwrap().clone(),
                    Some(Box::new(Value::NumberValue(Number::new(i)))),
                );
                i += step_value.value;

                let value = result.register(self.visit(node.body_node.clone(), context));

                if result.should_return()
                    && result.loop_should_continue == false
                    && result.loop_should_break == false
                {
                    return result;
                }

                if result.loop_should_continue {
                    continue;
                }

                if result.loop_should_break {
                    break;
                }

                let value = value.unwrap();

                elements.push(Some(value));
            }
        } else {
            while i > end_value.value {
                context.symbol_table.as_mut().unwrap().borrow_mut().set(
                    node.var_name_token.value.as_ref().unwrap().clone(),
                    Some(Box::new(Value::NumberValue(Number::new(i)))),
                );
                i += step_value.value;

                let value = result.register(self.visit(node.body_node.clone(), context));

                if result.should_return()
                    && result.loop_should_continue == false
                    && result.loop_should_break == false
                {
                    return result;
                }

                if result.loop_should_continue {
                    continue;
                }

                if result.loop_should_break {
                    break;
                }

                let value = value.unwrap();

                elements.push(Some(value));
            }
        }

        result.success(if node.should_return_null {
            Some(Number::null_value())
        } else {
            Some(
                Value::ListValue(List::new(elements))
                    .set_context(Some(context.clone()))
                    .set_position(node.pos_start.clone(), node.pos_end.clone()),
            )
        })
    }

    pub fn visit_while_node(&mut self, node: &WhileNode, context: &mut Context) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let mut elements: Vec<Option<Box<Value>>> = Vec::new();

        loop {
            let condition = result.register(self.visit(node.condition_node.clone(), context));

            if result.should_return() {
                return result;
            }

            let condition = condition.unwrap();

            if !condition.is_true() {
                break;
            }

            let value = result.register(self.visit(node.body_node.clone(), context));

            if result.should_return()
                && result.loop_should_continue == false
                && result.loop_should_break == false
            {
                return result;
            }

            if result.loop_should_continue {
                continue;
            }

            if result.loop_should_break {
                break;
            }

            let value = value.unwrap();

            elements.push(Some(value))
        }

        result.success(if node.should_return_null {
            Some(Number::null_value())
        } else {
            Some(
                Value::ListValue(List::new(elements))
                    .set_context(Some(context.clone()))
                    .set_position(node.pos_start.clone(), node.pos_end.clone()),
            )
        })
    }

    pub fn visit_function_definition_node(
        &mut self,
        node: &FunctionDefinitionNode,
        context: &mut Context,
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

        for arg_name in &node.arg_name_tokens {
            arg_names.push(arg_name.value.as_ref().unwrap().clone());
        }

        let func_value = Value::FunctionValue(Function::new(
            func_name.clone(),
            body_node,
            arg_names,
            node.should_auto_return,
        ))
        .set_context(Some(context.clone()))
        .set_position(node.pos_start.clone(), node.pos_end.clone());

        if !&func_name.is_empty() {
            context
                .symbol_table
                .as_mut()
                .unwrap()
                .borrow_mut()
                .set(func_name, Some(func_value.clone()));
        }

        result.success(Some(func_value))
    }

    pub fn visit_call_node(&mut self, node: &CallNode, context: &mut Context) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let mut args: Vec<Box<Value>> = Vec::new();

        let value_to_call = result.register(self.visit(node.node_to_call.clone(), context));

        if result.should_return() {
            return result;
        }

        let value_to_call = value_to_call
            .unwrap()
            .set_position(node.pos_start.clone(), node.pos_end.clone());

        for arg_node in &node.arg_nodes {
            let arg = result.register(self.visit(arg_node.to_owned(), context));

            if result.should_return() {
                return result;
            }

            let arg = arg.unwrap();

            args.push(arg);
        }

        let return_value = result.register(match value_to_call.as_ref() {
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

    pub fn visit_binary_operator_node(
        &mut self,
        node: &BinaryOperatorNode,
        context: &mut Context,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let left = result.register(self.visit(node.left_node.clone(), context));

        if result.should_return() {
            return result;
        }

        let mut left = left.unwrap();

        let right = result.register(self.visit(node.right_node.clone(), context));

        if result.should_return() {
            return result;
        }

        let right = right.unwrap();

        let (mut number, mut error): (Option<Box<Value>>, Option<StandardError>) = (None, None);

        if node.op_token.token_type == TokenType::TT_PLUS {
            (number, error) = left.perform_operation("+", right);
        } else if node.op_token.token_type == TokenType::TT_MINUS {
            (number, error) = left.perform_operation("-", right);
        } else if node.op_token.token_type == TokenType::TT_MUL {
            (number, error) = left.perform_operation("*", right);
        } else if node.op_token.token_type == TokenType::TT_DIV {
            (number, error) = left.perform_operation("/", right);
        } else if node.op_token.token_type == TokenType::TT_POW {
            (number, error) = left.perform_operation("^", right);
        } else if node.op_token.token_type == TokenType::TT_GT {
            (number, error) = left.perform_operation(">", right);
        } else if node.op_token.token_type == TokenType::TT_LT {
            (number, error) = left.perform_operation("<", right);
        } else if node.op_token.token_type == TokenType::TT_EE {
            (number, error) = left.perform_operation("==", right);
        } else if node.op_token.token_type == TokenType::TT_NE {
            (number, error) = left.perform_operation("!=", right);
        } else if node.op_token.token_type == TokenType::TT_LTE {
            (number, error) = left.perform_operation("<=", right);
        } else if node.op_token.token_type == TokenType::TT_GTE {
            (number, error) = left.perform_operation(">=", right);
        } else if node.op_token.matches(TokenType::TT_KEYWORD, Some("and")) {
            (number, error) = left.perform_operation("and", right);
        } else if node.op_token.matches(TokenType::TT_KEYWORD, Some("or")) {
            (number, error) = left.perform_operation("or", right);
        } else {
            (number, error) = left.perform_operation("", right);
        }

        if error.is_some() {
            return result.failure(error);
        } else {
            if number.is_some() {
                return result.success(Some(
                    number
                        .unwrap()
                        .set_position(node.pos_start.clone(), node.pos_end.clone()),
                ));
            } else {
                return result.success(None);
            }
        }
    }

    pub fn visit_unary_operator_node(
        &mut self,
        node: &UnaryOperatorNode,
        context: &mut Context,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let value = result
            .register(self.visit(node.node.clone(), context));

        if result.should_return() {
            return result;
        }

        let mut value = value.unwrap();

        let (mut number, mut error): (Option<Box<Value>>, Option<StandardError>) = (None, None);

        if node.op_token.token_type == TokenType::TT_MINUS {
            (number, error) =
                value.perform_operation("*", Box::new(Value::NumberValue(Number::new(-1.0))));
        } else if node
            .op_token
            .matches(TokenType::TT_KEYWORD, Some("not"))
        {
            (number, error) = value
                .perform_operation("not", Box::new(Value::NumberValue(Number::new(0.0))))
        }

        if error.is_some() {
            return result.failure(error);
        } else {
            if number.is_some() {
                return result.success(Some(
                    number
                        .unwrap()
                        .set_position(node.pos_start.clone(), node.pos_end.clone()),
                ));
            } else {
                return result.success(None);
            }
        }
    }

    pub fn visit_return_node(&mut self, node: &ReturnNode, context: &mut Context) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let mut value: Option<Box<Value>> = None;

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

    pub fn visit_continue_node(
        &mut self,
        node: &ContinueNode,
        context: &mut Context,
    ) -> RuntimeResult {
        RuntimeResult::new().success_continue()
    }

    pub fn visit_break_node(&mut self, node: &BreakNode, context: &mut Context) -> RuntimeResult {
        RuntimeResult::new().success_break()
    }
}
