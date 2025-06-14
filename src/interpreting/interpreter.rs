use crate::{
    errors::standard_error::StandardError,
    interpreting::{context::Context, runtime_result::RuntimeResult, symbol_table::SymbolTable},
    lexing::token_type::TokenType,
    nodes::{
        ast_node::AstNode, binary_operator_node::BinaryOperatorNode, list_node::ListNode,
        number_node::NumberNode, string_node::StringNode, unary_operator_node::UnaryOperatorNode,
        variable_access_node::VariableAccessNode, variable_assign_node::VariableAssignNode,
    },
    values::{list::List, number::Number, string::StringObj, value::Value},
};

pub struct Interpreter {
    pub global_symbol_table: SymbolTable,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            global_symbol_table: SymbolTable::new(None),
        }
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
            AstNode::BinaryOperator(node) => {
                return self.visit_binary_operator_node(&node, context);
            }
            AstNode::UnaryOperator(node) => {
                return self.visit_unary_operator_node(&node, context);
            }
            _ => {
                panic!("CRITICAL ERROR: NO METHOD DEFINED FOR NODE TYPE {:?}", node);
            }
        }
    }

    pub fn visit_number_node(&self, node: &NumberNode, context: &mut Context) -> RuntimeResult {
        let value: isize = node.token.value.as_ref().unwrap().parse().unwrap();

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
            .get(var_name.as_str())
            .clone();

        if value.is_none() {
            return result.failure(Some(StandardError::new(
                format!("variable name '{}' is undefined", var_name).to_string(),
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

    pub fn visit_binary_operator_node(
        &mut self,
        node: &BinaryOperatorNode,
        context: &mut Context,
    ) -> RuntimeResult {
        let mut result = RuntimeResult::new();
        let mut left = result
            .register(self.visit(node.left_node.clone(), context))
            .unwrap();

        if result.should_return() {
            return result;
        }

        let right = result
            .register(self.visit(node.right_node.clone(), context))
            .unwrap();

        if result.should_return() {
            return result;
        }

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
        let mut value = result
            .register(self.visit(node.node.clone(), context))
            .unwrap();

        if result.should_return() {
            return result;
        }

        let (mut number, mut error): (Option<Box<Value>>, Option<StandardError>) = (None, None);

        if node.op_token.token_type == TokenType::TT_MINUS {
            (number, error) =
                value.perform_operation("*", Box::new(Value::NumberValue(Number::new(-1))));
        } else if node
            .op_token
            .matches(TokenType::TT_KEYWORD, Some("oppositeof"))
        {
            (number, error) =
                value.perform_operation("oppositeof", Box::new(Value::NumberValue(Number::new(0))))
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
}

//     def visit_VariableAccessNode(self, node: VariableAccessNode, context):
//         result = RuntimeResult()
//         var_name = node.var_name_token.value
//         value = context.symbol_table.get(var_name)

//         if not value:
//             return result.failure(
//                 RunTimeError(node.pos_start,
//                              node.pos_end,
//                              f'"{var_name}" is not defined',
//                              context))

//         value = value.copy().setPos(node.pos_start, node.pos_end).setContext(context)
//         return result.success(value)

//     def visit_VariableAssignNode(self, node: VariableAssignNode, context):
//         result = RuntimeResult()
//         var_name = node.var_name_token.value
//         value = result.register(self.visit(node.value_node, context))

//         if result.shouldReturn():
//             return result

//         context.symbol_table.set(var_name, value)
//         return result.success(value)

//     def visit_BinaryOperatorNode(self, node: BinaryOperatorNode, context):
//         result = RuntimeResult()
//         left = result.register(self.visit(node.left_node, context))

//         if result.shouldReturn():
//             return result

//         right = result.register(self.visit(node.right_node, context))

//         if result.shouldReturn():
//             return result

//         if node.op_token.type == TT_PLUS:
//             number, error = left.addedTo(right)
//         elif node.op_token.type == TT_MINUS:
//             number, error = left.subtractedBy(right)
//         elif node.op_token.type == TT_MUL:
//             number, error = left.multipliedBy(right)
//         elif node.op_token.type == TT_DIV:
//             number, error = left.dividedBy(right)
//         elif node.op_token.type == TT_POW:
//             number, error = left.poweredBy(right)
//         elif node.op_token.type == TT_EE:
//             number, error = left.getComparisonEq(right)
//         elif node.op_token.type == TT_NE:
//             number, error = left.getComparisonNe(right)
//         elif node.op_token.type == TT_LT:
//             number, error = left.getComparisonLt(right)
//         elif node.op_token.type == TT_GT:
//             number, error = left.getComparisonGt(right)
//         elif node.op_token.type == TT_LTE:
//             number, error = left.getComparisonLte(right)
//         elif node.op_token.type == TT_GTE:
//             number, error = left.getComparisonGte(right)
//         elif node.op_token.matches(TT_KEYWORD, 'and'):
//             number, error = left.andedBy(right)
//         elif node.op_token.matches(TT_KEYWORD, 'or'):
//             number, error = left.oredBy(right)

//         if error:
//             return result.failure(error)

//         else:
//             return result.success(number.setPos(node.pos_start, node.pos_end))

//     def visit_UnaryOperatorNode(self, node: UnaryOperatorNode, context):
//         result = RuntimeResult()
//         number = result.register(self.visit(node.node, context))

//         if result.shouldReturn():
//             return result

//         error = None

//         if node.op_token.type == TT_MINUS:
//             number, error = number.multipliedBy(Number(-1))

//         elif node.op_token.matches(TT_KEYWORD, 'oppositeof'):
//             number, error = number.notted()

//         if error:
//             return result.failure(error)

//         else:
//             return result.success(number.setPos(node.pos_start, node.pos_end))

//     def visit_IfNode(self, node, context):
//         result = RuntimeResult()

//         for condition, expr, should_return_null in node.cases:
//             condition_value = result.register(self.visit(condition, context))

//             if result.shouldReturn():
//                 return result

//             if condition_value.isTrue():
//                 expr_value = result.register(self.visit(expr, context))

//                 if result.shouldReturn():
//                     return result

//                 return result.success(Number.null if should_return_null else expr_value)

//         if node.else_case:
//             expr, should_return_null = node.else_case
//             else_value = result.register(self.visit(expr, context))

//             if result.shouldReturn():
//                 return result

//             return result.success(Number.null if should_return_null else else_value)

//         return result.success(Number.null)

//     def visit_ForNode(self, node, context):
//         result = RuntimeResult()
//         elements = []

//         start_value = result.register(self.visit(node.start_value_node, context))

//         if result.shouldReturn():
//             return result

//         end_value = result.register(self.visit(node.end_value_node, context))

//         if result.shouldReturn():
//             return result

//         if node.step_value_node:
//             step_value = result.register(self.visit(node.step_value_node, context))

//             if result.shouldReturn():
//                 return result

//         else:
//             step_value = Number(1)

//         i = start_value.value

//         if step_value.value >= 0:
//             condition = lambda: i < end_value.value

//         else:
//             condition = lambda: i > end_value.value

//         while condition():
//             context.symbol_table.set(node.var_name_token.value, Number(i))
//             i += step_value.value

//             value = result.register(self.visit(node.body_node, context))

//             if result.shouldReturn() and result.loop_should_continue == False and result.loop_should_break == False:
//                 return result

//             if result.loop_should_continue:
//                 continue

//             if result.loop_should_break:
//                 break

//             elements.append(value)

//         return result.success(
//             Number.null if node.should_return_null else List(elements).setContext(
//                 context).setPos(node.pos_start, node.pos_end)
//         )

//     def visit_WhileNode(self, node, context):
//         result = RuntimeResult()
//         elements = []

//         while True:
//             condition = result.register(self.visit(node.condition_node, context))

//             if result.shouldReturn():
//                 return result

//             if not condition.isTrue():
//                 break

//             value = result.register(self.visit(node.body_node, context))

//             if result.shouldReturn() and result.loop_should_continue is False and result.loop_should_break is False:
//                 return result

//             if result.loop_should_continue:
//                 continue

//             if result.loop_should_break:
//                 break

//             elements.append(value)

//         return result.success(Number.null if node.should_return_null else
//                               List(elements).setContext(context).setPos(node.pos_start, node.pos_end)
//                               )

//     def visit_FunctionDefinitionNode(self, node, context):
//         result = RuntimeResult()

//         func_name = node.var_name_token.value if node.var_name_token else None
//         body_node = node.body_node
//         arg_names = [arg_name.value for arg_name in node.arg_name_tokens]
//         func_value = Function(func_name, body_node, arg_names, node.should_auto_return).setContext(context).setPos(
//             node.pos_start,
//             node.pos_end)

//         if node.var_name_token:
//             context.symbol_table.set(func_name, func_value)

//         return result.success(func_value)

//     def visit_CallNode(self, node, context):
//         result = RuntimeResult()
//         args = []

//         value_to_call = result.register(self.visit(node.node_to_call, context))

//         if result.shouldReturn():
//             return result

//         value_to_call = value_to_call.copy().setPos(node.pos_start, node.pos_end)

//         for arg_node in node.arg_nodes:
//             args.append(result.register(self.visit(arg_node, context)))

//             if result.shouldReturn():
//                 return result

//         return_value = result.register(value_to_call.execute(args))

//         if result.shouldReturn():
//             return result

//         return_value = return_value.copy().setPos(node.pos_start, node.pos_end).setContext(context)

//         return result.success(return_value)

//     def visit_ReturnNode(self, node, context):
//         result = RuntimeResult()

//         if node.node_to_return:
//             value = result.register(self.visit(node.node_to_return, context))

//             if result.shouldReturn():
//                 return result

//         else:
//             value = Number.null

//         return result.successReturn(value)

//     def visit_ContinueNode(self, node, context):
//         return RuntimeResult().successContinue()

//     def visit_BreakNode(self, node, context):
//         return RuntimeResult().successBreak()
