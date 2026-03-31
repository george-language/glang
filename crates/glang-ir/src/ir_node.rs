use glang_attributes::{Position, Span};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IrSpan {
    pub filename: String,
    pub start: Position,
    pub end: Position,
}

impl IrSpan {
    pub fn new(filename: &str, start: Position, end: Position) -> Self {
        Self {
            filename: filename.to_owned(),
            start,
            end,
        }
    }

    pub fn from_span(span: Span) -> IrSpan {
        Self {
            filename: span.filename.to_string_lossy().to_string(),
            start: span.start,
            end: span.end,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum IrNode {
    BinaryOperator(IrBinaryOperatorNode),
    Break(IrBreakNode),
    Call(IrCallNode),
    ConstAssign(IrConstAssignNode),
    Continue(IrContinueNode),
    For(IrForNode),
    FunctionDefinition(IrFunctionDefinitionNode),
    If(IrIfNode),
    Import(IrImportNode),
    List(IrListNode),
    Number(IrNumberNode),
    Return(IrReturnNode),
    Strings(IrStringNode),
    TryExcept(IrTryExceptNode),
    UnaryOperator(IrUnaryOperatorNode),
    VariableAccess(IrVariableAccessNode),
    VariableAssign(IrVariableAssignNode),
    VariableReassign(IrVariableRessignNode),
    While(IrWhileNode),
}

impl IrNode {
    pub fn span(&self) -> IrSpan {
        match self {
            IrNode::BinaryOperator(node) => node.span.clone(),
            IrNode::Break(node) => node.span.clone(),
            IrNode::Call(node) => node.span.clone(),
            IrNode::ConstAssign(node) => node.span.clone(),
            IrNode::Continue(node) => node.span.clone(),
            IrNode::For(node) => node.span.clone(),
            IrNode::FunctionDefinition(node) => node.span.clone(),
            IrNode::If(node) => node.span.clone(),
            IrNode::Import(node) => node.span.clone(),
            IrNode::List(node) => node.span.clone(),
            IrNode::Number(node) => node.span.clone(),
            IrNode::Return(node) => node.span.clone(),
            IrNode::Strings(node) => node.span.clone(),
            IrNode::TryExcept(node) => node.span.clone(),
            IrNode::UnaryOperator(node) => node.span.clone(),
            IrNode::VariableAccess(node) => node.span.clone(),
            IrNode::VariableAssign(node) => node.span.clone(),
            IrNode::VariableReassign(node) => node.span.clone(),
            IrNode::While(node) => node.span.clone(),
        }
    }

    pub fn position_start(&self) -> Position {
        match self {
            IrNode::BinaryOperator(node) => node.span.start.clone(),
            IrNode::Break(node) => node.span.start.clone(),
            IrNode::Call(node) => node.span.start.clone(),
            IrNode::ConstAssign(node) => node.span.start.clone(),
            IrNode::Continue(node) => node.span.start.clone(),
            IrNode::For(node) => node.span.start.clone(),
            IrNode::FunctionDefinition(node) => node.span.start.clone(),
            IrNode::If(node) => node.span.start.clone(),
            IrNode::Import(node) => node.span.start.clone(),
            IrNode::List(node) => node.span.start.clone(),
            IrNode::Number(node) => node.span.start.clone(),
            IrNode::Return(node) => node.span.start.clone(),
            IrNode::Strings(node) => node.span.start.clone(),
            IrNode::TryExcept(node) => node.span.start.clone(),
            IrNode::UnaryOperator(node) => node.span.start.clone(),
            IrNode::VariableAccess(node) => node.span.start.clone(),
            IrNode::VariableAssign(node) => node.span.start.clone(),
            IrNode::VariableReassign(node) => node.span.start.clone(),
            IrNode::While(node) => node.span.start.clone(),
        }
    }

    pub fn position_end(&self) -> Position {
        match self {
            IrNode::BinaryOperator(node) => node.span.end.clone(),
            IrNode::Break(node) => node.span.end.clone(),
            IrNode::Call(node) => node.span.end.clone(),
            IrNode::ConstAssign(node) => node.span.end.clone(),
            IrNode::Continue(node) => node.span.end.clone(),
            IrNode::For(node) => node.span.end.clone(),
            IrNode::FunctionDefinition(node) => node.span.end.clone(),
            IrNode::If(node) => node.span.end.clone(),
            IrNode::Import(node) => node.span.end.clone(),
            IrNode::List(node) => node.span.end.clone(),
            IrNode::Number(node) => node.span.end.clone(),
            IrNode::Return(node) => node.span.end.clone(),
            IrNode::Strings(node) => node.span.end.clone(),
            IrNode::TryExcept(node) => node.span.end.clone(),
            IrNode::UnaryOperator(node) => node.span.end.clone(),
            IrNode::VariableAccess(node) => node.span.end.clone(),
            IrNode::VariableAssign(node) => node.span.end.clone(),
            IrNode::VariableReassign(node) => node.span.end.clone(),
            IrNode::While(node) => node.span.end.clone(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IrBinaryOperatorNode {
    pub left: Box<IrNode>,
    pub right: Box<IrNode>,
    pub op: String,
    pub span: IrSpan,
}

impl IrBinaryOperatorNode {
    pub fn new(left: Box<IrNode>, right: Box<IrNode>, op: &str, span: IrSpan) -> Self {
        Self {
            left,
            right,
            op: op.to_string(),
            span,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IrBreakNode {
    pub span: IrSpan,
}

impl IrBreakNode {
    pub fn new(span: IrSpan) -> Self {
        Self { span }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IrCallNode {
    pub node_to_call: Box<IrNode>,
    pub arg_nodes: Vec<Box<IrNode>>,
    pub span: IrSpan,
}

impl IrCallNode {
    pub fn new(node_to_call: Box<IrNode>, arg_nodes: Vec<Box<IrNode>>, span: IrSpan) -> Self {
        Self {
            node_to_call,
            arg_nodes,
            span,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IrConstAssignNode {
    pub const_name: String,
    pub value_node: Box<IrNode>,
    pub span: IrSpan,
}

impl IrConstAssignNode {
    pub fn new(const_name: &str, value_node: Box<IrNode>, span: IrSpan) -> Self {
        Self {
            const_name: const_name.to_owned(),
            value_node,
            span,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IrContinueNode {
    pub span: IrSpan,
}

impl IrContinueNode {
    pub fn new(span: IrSpan) -> Self {
        Self { span }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IrForNode {
    pub iterator_name: String,
    pub start_value_node: Box<IrNode>,
    pub end_value_node: Box<IrNode>,
    pub step_value_node: Option<Box<IrNode>>,
    pub body_node: Box<IrNode>,
    pub span: IrSpan,
}

impl IrForNode {
    pub fn new(
        var_name: &str,
        start_value_node: Box<IrNode>,
        end_value_node: Box<IrNode>,
        step_value_node: Option<Box<IrNode>>,
        body_node: Box<IrNode>,
        span: IrSpan,
    ) -> Self {
        Self {
            iterator_name: var_name.to_owned(),
            start_value_node,
            end_value_node,
            step_value_node,
            body_node,
            span,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IrFunctionDefinitionNode {
    pub variable_name: Option<String>,
    pub arg_name_tokens: Vec<String>,
    pub body_node: Box<IrNode>,
    pub should_auto_return: bool,
    pub span: IrSpan,
}

impl IrFunctionDefinitionNode {
    pub fn new(
        variable_name: Option<String>,
        arg_name_tokens: Vec<String>,
        body_node: Box<IrNode>,
        should_auto_return: bool,
        span: IrSpan,
    ) -> Self {
        Self {
            variable_name,
            arg_name_tokens,
            body_node,
            should_auto_return,
            span,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IrIfNode {
    pub cases: Vec<(IrNode, IrNode, bool)>,
    pub else_case: Option<(Box<IrNode>, bool)>,
    pub span: IrSpan,
}

impl IrIfNode {
    pub fn new(
        cases: &[(IrNode, IrNode, bool)],
        else_case: Option<(Box<IrNode>, bool)>,
        span: IrSpan,
    ) -> Self {
        Self {
            cases: cases.to_vec(),
            else_case,
            span,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IrImportNode {
    pub node_to_import: Box<IrNode>,
    pub span: IrSpan,
}

impl IrImportNode {
    pub fn new(node_to_import: Box<IrNode>, span: IrSpan) -> Self {
        Self {
            node_to_import,
            span,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IrListNode {
    pub element_nodes: Vec<IrNode>,
    pub span: IrSpan,
}

impl IrListNode {
    pub fn new(element_nodes: Vec<IrNode>, span: IrSpan) -> Self {
        Self {
            element_nodes,
            span,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IrNumberNode {
    pub value: f64,
    pub span: IrSpan,
}

impl IrNumberNode {
    pub fn new(value: f64, span: IrSpan) -> Self {
        Self { value, span }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IrReturnNode {
    pub span: IrSpan,
}

impl IrReturnNode {
    pub fn new(span: IrSpan) -> Self {
        Self { span }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IrStringNode {
    pub value: String,
    pub span: IrSpan,
}

impl IrStringNode {
    pub fn new(value: String, span: IrSpan) -> Self {
        Self { value, span }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IrTryExceptNode {
    pub try_body_node: Box<IrNode>,
    pub except_body_node: Box<IrNode>,
    pub passed_error: String,
    pub span: IrSpan,
}

impl IrTryExceptNode {
    pub fn new(
        try_body_node: Box<IrNode>,
        except_body_node: Box<IrNode>,
        passed_error: String,
        span: IrSpan,
    ) -> Self {
        Self {
            try_body_node,
            except_body_node,
            passed_error,
            span,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IrUnaryOperatorNode {
    pub operator: String,
    pub node: Box<IrNode>,
    pub span: IrSpan,
}

impl IrUnaryOperatorNode {
    pub fn new(operator: String, node: Box<IrNode>, span: IrSpan) -> Self {
        Self {
            operator,
            node,
            span,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IrVariableAccessNode {
    pub name: String,
    pub span: IrSpan,
}

impl IrVariableAccessNode {
    pub fn new(name: String, span: IrSpan) -> Self {
        Self { name, span }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IrVariableAssignNode {
    pub name: String,
    pub value_node: Box<IrNode>,
    pub span: IrSpan,
}

impl IrVariableAssignNode {
    pub fn new(name: String, value_node: Box<IrNode>, span: IrSpan) -> Self {
        Self {
            name,
            value_node,
            span,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IrVariableRessignNode {
    pub name: String,
    pub value_node: Box<IrNode>,
    pub span: IrSpan,
}

impl IrVariableRessignNode {
    pub fn new(name: String, value_node: Box<IrNode>, span: IrSpan) -> Self {
        Self {
            name,
            value_node,
            span,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IrWhileNode {
    pub condition_node: Box<IrNode>,
    pub body_node: Box<IrNode>,
    pub span: IrSpan,
}

impl IrWhileNode {
    pub fn new(condition_node: Box<IrNode>, body_node: Box<IrNode>, span: IrSpan) -> Self {
        Self {
            condition_node,
            body_node,
            span,
        }
    }
}
