use crate::{
    lexing::position::Position,
    nodes::{
        binary_operator_node::BinaryOperatorNode, break_node::BreakNode, call_node::CallNode,
        const_assign_node::ConstAssignNode, continue_node::ContinueNode, for_node::ForNode,
        function_definition_node::FunctionDefinitionNode, if_node::IfNode, import_node::ImportNode,
        list_node::ListNode, number_node::NumberNode, return_node::ReturnNode,
        string_node::StringNode, try_except_node::TryExceptNode,
        unary_operator_node::UnaryOperatorNode, variable_access_node::VariableAccessNode,
        variable_assign_node::VariableAssignNode, while_node::WhileNode,
    },
};

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
    While(WhileNode),
}

impl AstNode {
    pub fn position_start(&self) -> Option<Position> {
        match self {
            AstNode::BinaryOperator(node) => node.pos_start.clone(),
            AstNode::Break(node) => node.pos_start.clone(),
            AstNode::Call(node) => node.pos_start.clone(),
            AstNode::ConstAssign(node) => node.pos_start.clone(),
            AstNode::Continue(node) => node.pos_start.clone(),
            AstNode::For(node) => node.pos_start.clone(),
            AstNode::FunctionDefinition(node) => node.pos_start.clone(),
            AstNode::If(node) => node.pos_start.clone(),
            AstNode::Import(node) => node.pos_start.clone(),
            AstNode::List(node) => node.pos_start.clone(),
            AstNode::Number(node) => node.pos_start.clone(),
            AstNode::Return(node) => node.pos_start.clone(),
            AstNode::Strings(node) => node.pos_start.clone(),
            AstNode::TryExcept(node) => node.pos_start.clone(),
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
            AstNode::ConstAssign(node) => node.pos_end.clone(),
            AstNode::Continue(node) => node.pos_end.clone(),
            AstNode::For(node) => node.pos_end.clone(),
            AstNode::FunctionDefinition(node) => node.pos_end.clone(),
            AstNode::If(node) => node.pos_end.clone(),
            AstNode::Import(node) => node.pos_end.clone(),
            AstNode::List(node) => node.pos_end.clone(),
            AstNode::Number(node) => node.pos_end.clone(),
            AstNode::Return(node) => node.pos_end.clone(),
            AstNode::Strings(node) => node.pos_end.clone(),
            AstNode::TryExcept(node) => node.pos_end.clone(),
            AstNode::UnaryOperator(node) => node.pos_end.clone(),
            AstNode::VariableAccess(node) => node.pos_end.clone(),
            AstNode::VariableAssign(node) => node.pos_end.clone(),
            AstNode::While(node) => node.pos_end.clone(),
        }
    }
}
