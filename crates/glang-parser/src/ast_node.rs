use glang_attributes::{Position, Span};
use glang_lexer::Token;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum AstNode {
    BinaryOperator(BinaryOperatorNode),
    Break(BreakNode),
    Call(CallNode),
    ConstAssign(ConstAssignNode),
    Continue(ContinueNode),
    For(ForNode),
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
    pub left_node: Box<AstNode>,
    pub op_token: Token,
    pub right_node: Box<AstNode>,
    pub span: Span,
}

impl BinaryOperatorNode {
    pub fn new(left_node: Box<AstNode>, op_token: Token, right_node: Box<AstNode>) -> Self {
        let pos_start = left_node.position_start();
        let pos_end = right_node.position_end();
        let filename = left_node.span().filename;

        Self {
            left_node,
            op_token,
            right_node,
            span: Span::new(&filename, pos_start, pos_end),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BreakNode {
    pub span: Span,
}

impl BreakNode {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}

#[derive(Debug, Clone)]
pub struct CallNode {
    pub node_to_call: Box<AstNode>,
    pub arg_nodes: Vec<Box<AstNode>>,
    pub span: Span,
}

impl CallNode {
    pub fn new(
        node_to_call: Box<AstNode>,
        arg_nodes: Vec<Box<AstNode>>,
        closing_bracket: Token,
    ) -> Self {
        Self {
            node_to_call: node_to_call.to_owned(),
            arg_nodes: arg_nodes.to_owned(),
            span: Span::new(
                &node_to_call.span().filename,
                node_to_call.position_start(),
                closing_bracket.span.end,
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConstAssignNode {
    pub const_name_token: Token,
    pub value_node: Box<AstNode>,
    pub span: Span,
}

impl ConstAssignNode {
    pub fn new(var_name_token: Token, value_node: Box<AstNode>) -> Self {
        Self {
            const_name_token: var_name_token.to_owned(),
            value_node,
            span: var_name_token.span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContinueNode {
    pub span: Span,
}

impl ContinueNode {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}

#[derive(Debug, Clone)]
pub struct ForNode {
    pub var_name_token: Token,
    pub start_value_node: Box<AstNode>,
    pub end_value_node: Box<AstNode>,
    pub step_value_node: Option<Box<AstNode>>,
    pub body_node: Box<AstNode>,
    pub span: Span,
}

impl ForNode {
    pub fn new(
        var_name_token: Token,
        start_value_node: Box<AstNode>,
        end_value_node: Box<AstNode>,
        step_value_node: Option<Box<AstNode>>,
        body_node: Box<AstNode>,
    ) -> Self {
        Self {
            var_name_token: var_name_token.to_owned(),
            start_value_node,
            end_value_node,
            step_value_node,
            body_node,
            span: var_name_token.span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionDefinitionNode {
    pub var_name_token: Option<Token>,
    pub arg_name_tokens: Rc<[Token]>,
    pub body_node: Box<AstNode>,
    pub should_auto_return: bool,
    pub span: Span,
}

impl FunctionDefinitionNode {
    pub fn new(
        var_name_token: Option<Token>,
        arg_name_tokens: &[Token],
        body_node: Box<AstNode>,
        should_auto_return: bool,
    ) -> Self {
        Self {
            var_name_token: var_name_token.to_owned(),
            arg_name_tokens: Rc::from(arg_name_tokens),
            body_node: body_node.to_owned(),
            should_auto_return,
            span: Span::new(
                &body_node.span().filename,
                if let Some(var_name) = var_name_token {
                    var_name.span.end
                } else if !arg_name_tokens.is_empty() {
                    arg_name_tokens[0].span.start.clone()
                } else {
                    body_node.position_start()
                },
                body_node.position_end(),
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IfNode {
    pub cases: Rc<[(Box<AstNode>, Box<AstNode>, bool)]>,
    pub else_case: Option<(Box<AstNode>, bool)>,
    pub span: Span,
}

impl IfNode {
    pub fn new(
        cases: &[(Box<AstNode>, Box<AstNode>, bool)],
        else_case: Option<(Box<AstNode>, bool)>,
    ) -> Self {
        Self {
            cases: Rc::from(cases),
            else_case: else_case.to_owned(),
            span: Span::new(
                &cases[0].0.span().filename,
                cases[0].0.position_start(),
                if else_case.is_none() {
                    cases[cases.len() - 1].0.position_start()
                } else {
                    else_case.unwrap().0.position_end()
                },
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImportNode {
    pub node_to_import: Box<AstNode>,
    pub span: Span,
}

impl ImportNode {
    pub fn new(node_to_import: Box<AstNode>) -> Self {
        Self {
            node_to_import: node_to_import.to_owned(),
            span: node_to_import.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListNode {
    pub element_nodes: Rc<[Box<AstNode>]>,
    pub span: Span,
}

impl ListNode {
    pub fn new(element_nodes: &[Box<AstNode>], span: Span) -> Self {
        Self {
            element_nodes: Rc::from(element_nodes),
            span,
        }
    }
}
#[derive(Debug, Clone)]
pub struct NumberNode {
    pub value: f64,
    pub span: Span,
}

impl NumberNode {
    pub fn new(token: Token) -> Self {
        Self {
            value: token.value.as_ref().unwrap().parse::<f64>().unwrap(),
            span: token.span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReturnNode {
    pub node_to_return: Option<Box<AstNode>>,
    pub span: Span,
}

impl ReturnNode {
    pub fn new(node: Option<Box<AstNode>>, span: Span) -> Self {
        Self {
            node_to_return: node,
            span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StringNode {
    pub token: Token,
    pub span: Span,
}

impl StringNode {
    pub fn new(token: Token) -> Self {
        Self {
            token: token.to_owned(),
            span: token.span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TryExceptNode {
    pub try_body_node: Box<AstNode>,
    pub except_body_node: Box<AstNode>,
    pub error_name_token: Token,
    pub span: Span,
}

impl TryExceptNode {
    pub fn new(
        try_body_node: Box<AstNode>,
        except_body_node: Box<AstNode>,
        error_name_token: Token,
    ) -> Self {
        Self {
            try_body_node: try_body_node.to_owned(),
            except_body_node: except_body_node.to_owned(),
            error_name_token,
            span: Span::new(
                &try_body_node.span().filename,
                try_body_node.position_start(),
                except_body_node.position_end(),
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnaryOperatorNode {
    pub op_token: Token,
    pub node: Box<AstNode>,
    pub span: Span,
}

impl UnaryOperatorNode {
    pub fn new(op_token: Token, node: Box<AstNode>) -> Self {
        let pos_end = node.position_end();
        let filename = node.span().filename;

        Self {
            op_token: op_token.to_owned(),
            node,
            span: Span::new(&filename, op_token.span.start, pos_end),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VariableAccessNode {
    pub var_name_token: Token,
    pub span: Span,
}

impl VariableAccessNode {
    pub fn new(var_name_token: Token) -> Self {
        Self {
            var_name_token: var_name_token.to_owned(),
            span: var_name_token.span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VariableAssignNode {
    pub var_name_token: Token,
    pub value_node: Box<AstNode>,
    pub span: Span,
}

impl VariableAssignNode {
    pub fn new(var_name_token: Token, value_node: Box<AstNode>) -> Self {
        Self {
            var_name_token: var_name_token.to_owned(),
            value_node,
            span: var_name_token.span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VariableRessignNode {
    pub var_name_token: Token,
    pub value_node: Box<AstNode>,
    pub span: Span,
}

impl VariableRessignNode {
    pub fn new(var_name_token: Token, value_node: Box<AstNode>) -> Self {
        Self {
            var_name_token: var_name_token.to_owned(),
            value_node,
            span: var_name_token.span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WhileNode {
    pub condition_node: Box<AstNode>,
    pub body_node: Box<AstNode>,
    pub span: Span,
}

impl WhileNode {
    pub fn new(condition_node: Box<AstNode>, body_node: Box<AstNode>) -> Self {
        Self {
            condition_node: condition_node.clone(),
            body_node: body_node.clone(),
            span: Span::new(
                &condition_node.span().filename,
                condition_node.position_start(),
                body_node.position_end(),
            ),
        }
    }
}
