use crate::nodes::{
    binary_operator_node::BinaryOperatorNode, break_node::BreakNode, call_node::CallNode,
    const_assign_node::ConstAssignNode, continue_node::ContinueNode, for_node::ForNode,
    function_definition_node::FunctionDefinitionNode, if_node::IfNode, import_node::ImportNode,
    list_node::ListNode, number_node::NumberNode, return_node::ReturnNode, string_node::StringNode,
    try_except_node::TryExceptNode, unary_operator_node::UnaryOperatorNode,
    variable_access_node::VariableAccessNode, variable_assign_node::VariableAssignNode,
    variable_reassign_node::VariableRessignNode, while_node::WhileNode,
};
use glang_attributes::{Position, Span};

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
