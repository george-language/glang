use std::fmt::Display;

use crate::{
    lexing::position::Position,
    nodes::{
        binary_operator_node::BinaryOperatorNode, break_node::BreakNode, call_node::CallNode,
        continue_node::ContinueNode, for_node::ForNode,
        function_definition_node::FunctionDefinitionNode, if_node::IfNode, list_node::ListNode,
        number_node::NumberNode, return_node::ReturnNode, string_node::StringNode,
        unary_operator_node::UnaryOperatorNode, variable_access_node::VariableAccessNode,
        variable_assign_node::VariableAssignNode, while_node::WhileNode,
    },
};

#[derive(Debug, Clone)]
pub enum AstNode {
    BinaryOperator(BinaryOperatorNode),
    Break(BreakNode),
    Call(CallNode),
    Continue(ContinueNode),
    For(ForNode),
    FunctionDefinition(FunctionDefinitionNode),
    If(IfNode),
    List(ListNode),
    Number(NumberNode),
    Return(ReturnNode),
    Strings(StringNode),
    UnaryOperator(UnaryOperatorNode),
    VariableAccess(VariableAccessNode),
    VariableAssign(VariableAssignNode),
    While(WhileNode),
}

impl AstNode {
    pub fn position_start(&self) -> Option<Position> {
        match self {
            AstNode::BinaryOperator(node) => node.pos_start.clone(),
            AstNode::Break(node) => node.pos_start.clone(),
            AstNode::Call(node) => node.pos_start.clone(),
            AstNode::Continue(node) => node.pos_start.clone(),
            AstNode::For(node) => node.pos_start.clone(),
            AstNode::FunctionDefinition(node) => node.pos_start.clone(),
            AstNode::If(node) => node.pos_start.clone(),
            AstNode::List(node) => node.pos_start.clone(),
            AstNode::Number(node) => node.pos_start.clone(),
            AstNode::Return(node) => node.pos_start.clone(),
            AstNode::Strings(node) => node.pos_start.clone(),
            AstNode::UnaryOperator(node) => node.pos_start.clone(),
            AstNode::VariableAccess(node) => node.pos_start.clone(),
            AstNode::VariableAssign(node) => node.pos_start.clone(),
            AstNode::While(node) => node.pos_start.clone(),
        }
    }

    pub fn position_end(&self) -> Option<Position> {
        match self {
            AstNode::BinaryOperator(node) => node.pos_end.clone(),
            AstNode::Break(node) => node.pos_end.clone(),
            AstNode::Call(node) => node.pos_end.clone(),
            AstNode::Continue(node) => node.pos_end.clone(),
            AstNode::For(node) => node.pos_end.clone(),
            AstNode::FunctionDefinition(node) => node.pos_end.clone(),
            AstNode::If(node) => node.pos_end.clone(),
            AstNode::List(node) => node.pos_end.clone(),
            AstNode::Number(node) => node.pos_end.clone(),
            AstNode::Return(node) => node.pos_end.clone(),
            AstNode::Strings(node) => node.pos_end.clone(),
            AstNode::UnaryOperator(node) => node.pos_end.clone(),
            AstNode::VariableAccess(node) => node.pos_end.clone(),
            AstNode::VariableAssign(node) => node.pos_end.clone(),
            AstNode::While(node) => node.pos_end.clone(),
        }
    }
}

impl Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
