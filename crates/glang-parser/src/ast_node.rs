use glang_attributes::{Position, Span};
use glang_lexer::{Token, TokenType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeID(pub usize);

#[derive(Debug, Clone)]
pub struct AstArena {
    pub nodes: Vec<AstNode>,
}

impl AstArena {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn get(&self, id: NodeID) -> &AstNode {
        let node = &self.nodes[id.0];

        &node
    }

    pub fn binary_operator_node(&mut self, left: NodeID, op_token: Token, right: NodeID) -> NodeID {
        self.add(AstNode::BinaryOperator(BinaryOperatorNode {
            left_node: left,
            right_node: right,
            operator: (match op_token.token_type {
                TokenType::TT_PLUS => "+",
                TokenType::TT_MINUS => "-",
                TokenType::TT_MUL => "*",
                TokenType::TT_DIV => "/",
                TokenType::TT_POW => "^",
                TokenType::TT_MOD => "%",
                TokenType::TT_GT => ">",
                TokenType::TT_LT => "<",
                TokenType::TT_EE => "==",
                TokenType::TT_NE => "!=",
                TokenType::TT_LTE => "<=",
                TokenType::TT_GTE => ">=",
                _ if op_token.matches(TokenType::TT_KEYWORD, "and") => "and",
                _ if op_token.matches(TokenType::TT_KEYWORD, "or") => "or",
                _ => "None",
            })
            .to_owned(),
            span: Span::new(
                &self.span(left).filename,
                self.position_start(left),
                self.position_end(right),
            ),
        }))
    }

    pub fn break_node(&mut self, span: Span) -> NodeID {
        self.add(AstNode::Break(BreakNode { span }))
    }

    pub fn call_node(
        &mut self,
        node_to_call: NodeID,
        arg_nodes: Vec<NodeID>,
        closing_bracket: Token,
    ) -> NodeID {
        self.add(AstNode::Call(CallNode {
            node_to_call,
            arg_nodes,
            span: Span::new(
                &self.span(node_to_call).filename,
                self.position_start(node_to_call),
                closing_bracket.span.end,
            ),
        }))
    }

    pub fn const_assign_node(&mut self, var_name_token: Token, value_node: NodeID) -> NodeID {
        self.add(AstNode::ConstAssign(ConstAssignNode {
            name: var_name_token.value.unwrap(),
            value_node,
            span: var_name_token.span,
        }))
    }

    pub fn continue_node(&mut self, span: Span) -> NodeID {
        self.add(AstNode::Continue(ContinueNode { span }))
    }

    pub fn for_node(
        &mut self,
        var_name_token: Token,
        start_value_node: NodeID,
        end_value_node: NodeID,
        step_value_node: Option<NodeID>,
        body_node: NodeID,
    ) -> NodeID {
        self.add(AstNode::For(ForNode {
            iterator_name: var_name_token.value.unwrap(),
            start_value_node,
            end_value_node,
            step_value_node,
            body_node,
            span: var_name_token.span,
        }))
    }

    pub fn for_each_node(
        &mut self,
        var_name_token: Token,
        iterator: NodeID,
        body_node: NodeID,
    ) -> NodeID {
        self.add(AstNode::ForEach(ForEachNode {
            iterator_name: var_name_token.value.unwrap(),
            iterator_node: iterator,
            body_node,
            span: var_name_token.span,
        }))
    }

    pub fn function_definition_node(
        &mut self,
        var_name_token: Option<Token>,
        arg_name_tokens: &[Token],
        body_node: NodeID,
        should_auto_return: bool,
    ) -> NodeID {
        self.add(AstNode::FunctionDefinition(FunctionDefinitionNode {
            name: if let Some(ref tok) = var_name_token {
                tok.value.clone()
            } else {
                None
            },
            argument_names: arg_name_tokens.to_vec(),
            body_node: body_node,
            should_auto_return,
            span: Span::new(
                &self.span(body_node).filename,
                if let Some(var_name) = var_name_token {
                    var_name.span.end
                } else if !arg_name_tokens.is_empty() {
                    arg_name_tokens[0].span.start.clone()
                } else {
                    self.position_end(body_node)
                },
                self.position_end(body_node),
            ),
        }))
    }

    pub fn if_node(
        &mut self,
        cases: Vec<(NodeID, NodeID, bool)>,
        else_case: Option<(NodeID, bool)>,
    ) -> NodeID {
        self.add(AstNode::If(IfNode {
            cases: cases.to_owned(),
            else_case,
            span: Span::new(
                &self.span(cases[0].0).filename,
                self.position_start(cases[0].0),
                if else_case.is_none() {
                    self.position_start(cases[cases.len() - 1].0)
                } else {
                    self.position_end(else_case.unwrap().0)
                },
            ),
        }))
    }

    pub fn import_node(&mut self, node_to_import: NodeID) -> NodeID {
        self.add(AstNode::Import(ImportNode {
            node_to_import,
            span: self.span(node_to_import),
        }))
    }

    pub fn list_node(&mut self, element_nodes: Vec<NodeID>, span: Span) -> NodeID {
        self.add(AstNode::List(ListNode {
            element_nodes,
            span,
        }))
    }

    pub fn number_node(&mut self, token: Token) -> NodeID {
        self.add(AstNode::Number(NumberNode {
            value: token.value.as_ref().unwrap().parse::<f64>().unwrap(),
            span: token.span,
        }))
    }

    pub fn return_node(&mut self, node_to_return: Option<NodeID>, span: Span) -> NodeID {
        self.add(AstNode::Return(ReturnNode {
            node_to_return,
            span,
        }))
    }

    pub fn string_node(&mut self, token: Token) -> NodeID {
        self.add(AstNode::Strings(StringNode {
            value: token.value.unwrap(),
            span: token.span,
        }))
    }

    pub fn try_except_node(
        &mut self,
        try_body_node: NodeID,
        except_body_node: NodeID,
        error_name_token: Token,
    ) -> NodeID {
        self.add(AstNode::TryExcept(TryExceptNode {
            try_body_node: try_body_node,
            except_body_node: except_body_node,
            passed_error: error_name_token.value.unwrap(),
            span: Span::new(
                &self.span(try_body_node).filename,
                self.position_start(try_body_node),
                self.position_end(except_body_node),
            ),
        }))
    }

    pub fn unary_operator_node(&mut self, op_token: Token, node: NodeID) -> NodeID {
        self.add(AstNode::UnaryOperator(UnaryOperatorNode {
            operator: (match op_token.token_type {
                TokenType::TT_MINUS => "-1",
                TokenType::TT_KEYWORD => {
                    if op_token.matches(TokenType::TT_KEYWORD, "not") {
                        "not"
                    } else {
                        ""
                    }
                }
                _ => "",
            })
            .to_owned(),
            node,
            span: Span::new(
                &self.span(node).filename,
                op_token.span.start,
                self.position_end(node),
            ),
        }))
    }

    pub fn variable_access_node(&mut self, var_name_token: Token) -> NodeID {
        self.add(AstNode::VariableAccess(VariableAccessNode {
            name: var_name_token.value.unwrap(),
            span: var_name_token.span,
        }))
    }

    pub fn variable_assign_node(&mut self, var_name_token: Token, value_node: NodeID) -> NodeID {
        self.add(AstNode::VariableAssign(VariableAssignNode {
            name: var_name_token.value.unwrap(),
            value_node,
            span: var_name_token.span,
        }))
    }

    pub fn variable_reassign_node(&mut self, var_name_token: Token, value_node: NodeID) -> NodeID {
        self.add(AstNode::VariableReassign(VariableRessignNode {
            name: var_name_token.value.unwrap(),
            value_node,
            span: var_name_token.span,
        }))
    }

    pub fn while_node(&mut self, condition_node: NodeID, body_node: NodeID) -> NodeID {
        self.add(AstNode::While(WhileNode {
            condition_node: condition_node.clone(),
            body_node: body_node.clone(),
            span: Span::new(
                &self.span(condition_node).filename,
                self.position_start(condition_node),
                self.position_end(body_node),
            ),
        }))
    }

    pub fn add(&mut self, node: AstNode) -> NodeID {
        let id = NodeID(self.nodes.len());
        self.nodes.push(node);

        id
    }

    pub fn span(&self, id: NodeID) -> Span {
        self.get(id).span()
    }

    fn position_start(&self, id: NodeID) -> Position {
        self.get(id).position_start()
    }

    fn position_end(&self, id: NodeID) -> Position {
        self.get(id).position_end()
    }
}

#[derive(Debug, Clone)]
pub enum AstNode {
    BinaryOperator(BinaryOperatorNode),
    Break(BreakNode),
    Call(CallNode),
    ConstAssign(ConstAssignNode),
    Continue(ContinueNode),
    For(ForNode),
    ForEach(ForEachNode),
    FunctionDefinition(FunctionDefinitionNode),
    If(IfNode),
    Import(ImportNode),
    List(ListNode),
    Number(NumberNode),
    Return(ReturnNode),
    Strings(StringNode),
    TryExcept(TryExceptNode),
    UnaryOperator(UnaryOperatorNode),
    VariableAccess(VariableAccessNode),
    VariableAssign(VariableAssignNode),
    VariableReassign(VariableRessignNode),
    While(WhileNode),
}

impl AstNode {
    pub fn span(&self) -> Span {
        match self {
            AstNode::BinaryOperator(node) => node.span.clone(),
            AstNode::Break(node) => node.span.clone(),
            AstNode::Call(node) => node.span.clone(),
            AstNode::ConstAssign(node) => node.span.clone(),
            AstNode::Continue(node) => node.span.clone(),
            AstNode::For(node) => node.span.clone(),
            AstNode::ForEach(node) => node.span.clone(),
            AstNode::FunctionDefinition(node) => node.span.clone(),
            AstNode::If(node) => node.span.clone(),
            AstNode::Import(node) => node.span.clone(),
            AstNode::List(node) => node.span.clone(),
            AstNode::Number(node) => node.span.clone(),
            AstNode::Return(node) => node.span.clone(),
            AstNode::Strings(node) => node.span.clone(),
            AstNode::TryExcept(node) => node.span.clone(),
            AstNode::UnaryOperator(node) => node.span.clone(),
            AstNode::VariableAccess(node) => node.span.clone(),
            AstNode::VariableAssign(node) => node.span.clone(),
            AstNode::VariableReassign(node) => node.span.clone(),
            AstNode::While(node) => node.span.clone(),
        }
    }

    pub fn position_start(&self) -> Position {
        match self {
            AstNode::BinaryOperator(node) => node.span.start.clone(),
            AstNode::Break(node) => node.span.start.clone(),
            AstNode::Call(node) => node.span.start.clone(),
            AstNode::ConstAssign(node) => node.span.start.clone(),
            AstNode::Continue(node) => node.span.start.clone(),
            AstNode::For(node) => node.span.start.clone(),
            AstNode::ForEach(node) => node.span.start.clone(),
            AstNode::FunctionDefinition(node) => node.span.start.clone(),
            AstNode::If(node) => node.span.start.clone(),
            AstNode::Import(node) => node.span.start.clone(),
            AstNode::List(node) => node.span.start.clone(),
            AstNode::Number(node) => node.span.start.clone(),
            AstNode::Return(node) => node.span.start.clone(),
            AstNode::Strings(node) => node.span.start.clone(),
            AstNode::TryExcept(node) => node.span.start.clone(),
            AstNode::UnaryOperator(node) => node.span.start.clone(),
            AstNode::VariableAccess(node) => node.span.start.clone(),
            AstNode::VariableAssign(node) => node.span.start.clone(),
            AstNode::VariableReassign(node) => node.span.start.clone(),
            AstNode::While(node) => node.span.start.clone(),
        }
    }

    pub fn position_end(&self) -> Position {
        match self {
            AstNode::BinaryOperator(node) => node.span.end.clone(),
            AstNode::Break(node) => node.span.end.clone(),
            AstNode::Call(node) => node.span.end.clone(),
            AstNode::ConstAssign(node) => node.span.end.clone(),
            AstNode::Continue(node) => node.span.end.clone(),
            AstNode::For(node) => node.span.end.clone(),
            AstNode::ForEach(node) => node.span.end.clone(),
            AstNode::FunctionDefinition(node) => node.span.end.clone(),
            AstNode::If(node) => node.span.end.clone(),
            AstNode::Import(node) => node.span.end.clone(),
            AstNode::List(node) => node.span.end.clone(),
            AstNode::Number(node) => node.span.end.clone(),
            AstNode::Return(node) => node.span.end.clone(),
            AstNode::Strings(node) => node.span.end.clone(),
            AstNode::TryExcept(node) => node.span.end.clone(),
            AstNode::UnaryOperator(node) => node.span.end.clone(),
            AstNode::VariableAccess(node) => node.span.end.clone(),
            AstNode::VariableAssign(node) => node.span.end.clone(),
            AstNode::VariableReassign(node) => node.span.end.clone(),
            AstNode::While(node) => node.span.end.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryOperatorNode {
    pub left_node: NodeID,
    pub operator: String,
    pub right_node: NodeID,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct BreakNode {
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct CallNode {
    pub node_to_call: NodeID,
    pub arg_nodes: Vec<NodeID>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ConstAssignNode {
    pub name: String,
    pub value_node: NodeID,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ContinueNode {
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ForNode {
    pub iterator_name: String,
    pub start_value_node: NodeID,
    pub end_value_node: NodeID,
    pub step_value_node: Option<NodeID>,
    pub body_node: NodeID,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ForEachNode {
    pub iterator_name: String,
    pub iterator_node: NodeID,
    pub body_node: NodeID,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FunctionDefinitionNode {
    pub name: Option<String>,
    pub argument_names: Vec<Token>,
    pub body_node: NodeID,
    pub should_auto_return: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct IfNode {
    pub cases: Vec<(NodeID, NodeID, bool)>,
    pub else_case: Option<(NodeID, bool)>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ImportNode {
    pub node_to_import: NodeID,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ListNode {
    pub element_nodes: Vec<NodeID>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct NumberNode {
    pub value: f64,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ReturnNode {
    pub node_to_return: Option<NodeID>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct StringNode {
    pub value: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct TryExceptNode {
    pub try_body_node: NodeID,
    pub except_body_node: NodeID,
    pub passed_error: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct UnaryOperatorNode {
    pub operator: String,
    pub node: NodeID,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct VariableAccessNode {
    pub name: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct VariableAssignNode {
    pub name: String,
    pub value_node: NodeID,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct VariableRessignNode {
    pub name: String,
    pub value_node: NodeID,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct WhileNode {
    pub condition_node: NodeID,
    pub body_node: NodeID,
    pub span: Span,
}
