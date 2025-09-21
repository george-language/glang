use crate::{
    nodes::{
        ast_node::AstNode, binary_operator_node::BinaryOperatorNode, break_node::BreakNode,
        call_node::CallNode, const_assign_node::ConstAssignNode, continue_node::ContinueNode,
        for_node::ForNode, function_definition_node::FunctionDefinitionNode, if_node::IfNode,
        import_node::ImportNode, list_node::ListNode, number_node::NumberNode,
        return_node::ReturnNode, string_node::StringNode, try_except_node::TryExceptNode,
        unary_operator_node::UnaryOperatorNode, variable_access_node::VariableAccessNode,
        variable_assign_node::VariableAssignNode, variable_reassign_node::VariableRessignNode,
        while_node::WhileNode,
    },
    parse_result::ParseResult,
};
use glang_attributes::{Position, StandardError};
use glang_lexer::{Token, TokenType};
use std::rc::Rc;

pub struct Parser {
    pub tokens: Rc<[Token]>,
    pub token_index: isize,
    pub current_token: Option<Token>,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        let mut parser = Self {
            tokens: Rc::from(tokens),
            token_index: -1,
            current_token: None,
        };
        parser.advance();

        parser
    }

    fn advance(&mut self) -> Option<Token> {
        self.token_index += 1;
        self.update_current_token();

        self.current_token.clone()
    }

    fn reverse(&mut self, amount: usize) -> Option<Token> {
        self.token_index -= amount as isize;
        self.update_current_token();

        self.current_token.clone()
    }

    fn update_current_token(&mut self) {
        if self.token_index >= 0 && self.token_index < self.tokens.len() as isize {
            self.current_token = Some(self.tokens[self.token_index as usize].clone());
        }
    }

    fn current_token_copy(&mut self) -> Token {
        self.current_token.as_ref().unwrap().clone()
    }

    pub fn current_token_ref(&mut self) -> &Token {
        self.current_token.as_ref().unwrap()
    }

    fn next_token_copy(&mut self) -> Option<Token> {
        if self.token_index >= 0 && self.token_index + 1 < self.tokens.len() as isize {
            return Some(self.tokens[self.token_index as usize + 1].clone());
        }

        None
    }

    fn current_pos_start(&self) -> Rc<Position> {
        Rc::new(
            self.current_token
                .as_ref()
                .unwrap()
                .pos_start
                .as_ref()
                .unwrap()
                .clone(),
        )
    }

    fn current_pos_end(&self) -> Rc<Position> {
        Rc::new(
            self.current_token
                .as_ref()
                .unwrap()
                .pos_end
                .as_ref()
                .unwrap()
                .clone(),
        )
    }

    pub fn parse(&mut self) -> ParseResult {
        let mut parse_result = self.statements();

        if parse_result.error.is_some() && self.current_token_copy().token_type != TokenType::TT_EOF
        {
            return parse_result.failure(Some(StandardError::new(
                "expected keyword, object, function, expression",
                self.current_pos_start(),
                self.current_pos_end(),
                None,
            )));
        }

        parse_result
    }

    fn comparison_expr(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();

        if self
            .current_token_copy()
            .matches(TokenType::TT_KEYWORD, "not")
        {
            let op_token = self.current_token_copy();
            parse_result.register_advancement();
            self.advance();

            let node = parse_result
                .register(self.comparison_expr())
                .unwrap()
                .clone();

            if parse_result.error.is_some() {
                return parse_result;
            }

            return parse_result.success(Some(Box::new(AstNode::UnaryOperator(
                UnaryOperatorNode::new(op_token.clone(), node.clone()),
            ))));
        }

        let node = parse_result.register(self.binary_operator(
            "arithmetic_expr",
            &[
                (TokenType::TT_EE, ""),
                (TokenType::TT_NE, ""),
                (TokenType::TT_LT, ""),
                (TokenType::TT_GT, ""),
                (TokenType::TT_LTE, ""),
                (TokenType::TT_GTE, ""),
            ],
            None,
        ));

        if parse_result.error.is_some() {
            return parse_result.failure(Some(StandardError::new(
                "expected an object or operator",
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add one of the following: integer, float, identifier, 'not', '+', '-', '(', or '['"),
            )));
        }

        parse_result.success(node)
    }

    fn arithmetic_expr(&mut self) -> ParseResult {
        self.binary_operator(
            "term",
            &[(TokenType::TT_PLUS, ""), (TokenType::TT_MINUS, "")],
            None,
        )
    }

    fn list_expr(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let mut element_nodes: Vec<Box<AstNode>> = Vec::new();
        let pos_start = self.current_pos_start();

        if self.current_token_ref().token_type != TokenType::TT_LSQUARE {
            return parse_result.failure(Some(StandardError::new(
                "expected list initializing bracket",
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add a '[' to start the list"),
            )));
        }

        parse_result.register_advancement();
        self.advance();

        if self.current_token_ref().token_type == TokenType::TT_RSQUARE {
            parse_result.register_advancement();
            self.advance();
        } else {
            let element = parse_result.register(self.expr());

            if parse_result.error.is_some() {
                return parse_result.failure(Some(StandardError::new(
                    "expected closing bracket or list element",
                    self.current_pos_start(),
                    self.current_pos_end(),
                    Some("add a ']' to close the list or add a list element followed by a comma"),
                )));
            }

            element_nodes.push(element.unwrap());

            while self.current_token_ref().token_type == TokenType::TT_COMMA {
                parse_result.register_advancement();
                self.advance();

                if self.current_token_ref().token_type == TokenType::TT_RSQUARE {
                    break;
                }

                let element = parse_result.register(self.expr());

                if parse_result.error.is_some() {
                    return parse_result;
                }

                element_nodes.push(element.unwrap());
            }

            if self.current_token_ref().token_type != TokenType::TT_RSQUARE {
                return parse_result.failure(Some(StandardError::new(
                    "expected ']' or next list element",
                    self.current_pos_start(),
                    self.current_pos_end(),
                    Some("add a ']' to close the list or add a list element followed by a comma"),
                )));
            }

            parse_result.register_advancement();
            self.advance();
        }

        parse_result.success(Some(Box::new(AstNode::List(ListNode::new(
            &element_nodes,
            Some(pos_start),
            Some(self.current_pos_end()),
        )))))
    }

    fn if_expr(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let (if_parse_result, cases, else_case) = self.if_expr_cases("if");

        if if_parse_result.error.is_some() {
            return if_parse_result;
        }

        parse_result.success(Some(Box::new(AstNode::If(IfNode::new(&cases, else_case)))))
    }

    fn if_expr_b(
        &mut self,
    ) -> (
        ParseResult,
        Vec<(Box<AstNode>, Box<AstNode>, bool)>,
        Option<(Box<AstNode>, bool)>,
    ) {
        self.if_expr_cases("alsoif")
    }

    fn if_expr_c(&mut self) -> (ParseResult, Option<(Box<AstNode>, bool)>) {
        let mut parse_result = ParseResult::new();
        let mut else_case: Option<(Box<AstNode>, bool)> = None;

        if self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "otherwise")
        {
            parse_result.register_advancement();
            self.advance();

            if self.current_token_ref().token_type != TokenType::TT_LBRACKET {
                return (
                    parse_result.failure(Some(StandardError::new(
                        "expected '{'",
                        self.current_pos_start(),
                        self.current_pos_end(),
                        Some("add a '{' to define the body"),
                    ))),
                    None,
                );
            }

            parse_result.register_advancement();
            self.advance();

            let statements = parse_result.register(self.statements());

            if parse_result.error.is_some() {
                return (parse_result, None);
            }

            let body = (statements.unwrap(), true);

            else_case = Some(body);

            if self.current_token_ref().token_type != TokenType::TT_RBRACKET {
                return (
                    parse_result.failure(Some(StandardError::new(
                        "expected '}'",
                        self.current_pos_start(),
                        self.current_pos_end(),
                        Some("add a '}' to close the body"),
                    ))),
                    None,
                );
            }

            parse_result.register_advancement();
            self.advance();
        }

        (parse_result, else_case)
    }

    fn if_expr_b_or_c(
        &mut self,
    ) -> (
        ParseResult,
        Vec<(Box<AstNode>, Box<AstNode>, bool)>,
        Option<(Box<AstNode>, bool)>,
    ) {
        let mut parse_result = ParseResult::new();
        let mut cases: Vec<(Box<AstNode>, Box<AstNode>, bool)> = Vec::new();
        let mut else_case: Option<(Box<AstNode>, bool)> = None;

        while self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "alsoif")
        {
            let (if_parse_result, mut new_cases, new_else_case) = self.if_expr_b();

            if if_parse_result.error.is_some() {
                return (if_parse_result, Vec::new(), None);
            }

            parse_result.register(if_parse_result);

            cases.append(&mut new_cases);

            if new_else_case.is_some() {
                else_case = new_else_case;
                break;
            }
        }

        if self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "otherwise")
        {
            let (else_parse_result, new_else_case) = self.if_expr_c();

            if else_parse_result.error.is_some() {
                return (else_parse_result, Vec::new(), None);
            }

            parse_result.register(else_parse_result);
            else_case = new_else_case;
        }

        (parse_result, cases, else_case)
    }

    fn if_expr_cases(
        &mut self,
        keyword: &str,
    ) -> (
        ParseResult,
        Vec<(Box<AstNode>, Box<AstNode>, bool)>,
        Option<(Box<AstNode>, bool)>,
    ) {
        let mut parse_result = ParseResult::new();
        let mut cases: Vec<(Box<AstNode>, Box<AstNode>, bool)> = Vec::new();
        let mut else_case: Option<(Box<AstNode>, bool)> = None;

        if !self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, keyword)
        {
            return (
                parse_result.failure(Some(StandardError::new(
                    "expected keyword",
                    self.current_pos_start(),
                    self.current_pos_end(),
                    Some(format!("add the '{keyword}' keyword").as_str()),
                ))),
                Vec::new(),
                None,
            );
        }

        parse_result.register_advancement();
        self.advance();

        let condition = parse_result.register(self.statement());

        if parse_result.error.is_some() {
            return (parse_result, Vec::new(), None);
        }

        if self.current_token_ref().token_type != TokenType::TT_LBRACKET {
            return (
                parse_result.failure(Some(StandardError::new(
                    "expected '{'",
                    self.current_pos_start(),
                    self.current_pos_end(),
                    Some("add a '{' to define the body"),
                ))),
                Vec::new(),
                None,
            );
        }

        parse_result.register_advancement();
        self.advance();

        let statements = parse_result.register(self.statements());

        if parse_result.error.is_some() {
            return (parse_result, Vec::new(), None);
        }

        cases.push((condition.unwrap(), statements.unwrap(), true));

        if self.current_token_ref().token_type != TokenType::TT_RBRACKET {
            return (
                parse_result.failure(Some(StandardError::new(
                    "expected '}'",
                    self.current_pos_start(),
                    self.current_pos_end(),
                    Some("add a '}' to close the body"),
                ))),
                Vec::new(),
                None,
            );
        }

        parse_result.register_advancement();
        self.advance();

        let (if_parse_result, all_cases, else_clause) = self.if_expr_b_or_c();

        if if_parse_result.error.is_some() {
            return (if_parse_result, Vec::new(), None);
        }

        else_case = else_clause;
        cases.append(&mut all_cases.clone());

        (parse_result, cases, else_case)
    }

    fn for_expr(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();

        if !self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "walk")
        {
            return parse_result.failure(Some(StandardError::new(
                "expected keyword",
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add the 'walk' keyword to represent a for loop"),
            )));
        }

        parse_result.register_advancement();
        self.advance();

        if self.current_token_ref().token_type != TokenType::TT_IDENTIFIER {
            return parse_result.failure(Some(StandardError::new(
                "expected identifier",
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add an object name like 'i' to represent a for loop's iterator"),
            )));
        }

        let var_name = self.current_token_copy();

        parse_result.register_advancement();
        self.advance();

        if self.current_token_ref().token_type != TokenType::TT_EQ {
            return parse_result.failure(Some(StandardError::new(
                "expected '='",
                self.current_pos_start(),
                self.current_pos_end(),
                Some(
                    format!(
                        "add an '=' to set the value of the variable '{}'",
                        var_name.value.unwrap().clone()
                    )
                    .as_str(),
                ),
            )));
        }

        parse_result.register_advancement();
        self.advance();

        let start_value = parse_result.register(self.expr());

        if parse_result.error.is_some() {
            return parse_result;
        }

        if !self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "through")
        {
            return parse_result.failure(Some(StandardError::new(
                "expected 'through'",
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add the 'through' keyword to define a range 'n through n'"),
            )));
        }

        parse_result.register_advancement();
        self.advance();

        let end_value = parse_result.register(self.expr());

        if parse_result.error.is_some() {
            return parse_result;
        }

        let step_value: Option<Box<AstNode>>;

        if self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "step")
        {
            parse_result.register_advancement();
            self.advance();

            if self.current_token_ref().token_type != TokenType::TT_EQ {
                return parse_result.failure(Some(StandardError::new(
                    "expected '='",
                    self.current_pos_start(),
                    self.current_pos_end(),
                    Some("add an '=' to set the step amount"),
                )));
            }

            parse_result.register_advancement();
            self.advance();

            step_value = parse_result.register(self.expr());

            if parse_result.error.is_some() {
                return parse_result;
            }
        } else {
            step_value = None;
        }

        if self.current_token_ref().token_type != TokenType::TT_LBRACKET {
            return parse_result.failure(Some(StandardError::new(
                "expected '{'",
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add a '{' to define the body"),
            )));
        }

        parse_result.register_advancement();
        self.advance();

        let body = parse_result.register(self.statements());

        if parse_result.error.is_some() {
            return parse_result;
        }

        if self.current_token_ref().token_type != TokenType::TT_RBRACKET {
            return parse_result.failure(Some(StandardError::new(
                "expected '}'",
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add a '}' to close the body"),
            )));
        }

        parse_result.register_advancement();
        self.advance();

        parse_result.success(Some(Box::new(AstNode::For(ForNode::new(
            var_name,
            start_value.unwrap(),
            end_value.unwrap(),
            step_value,
            body.unwrap(),
        )))))
    }

    fn while_expr(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();

        if !self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "while")
        {
            return parse_result.failure(Some(StandardError::new(
                "expected keyword",
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add the 'while' keyword to represent a while loop"),
            )));
        }

        parse_result.register_advancement();
        self.advance();

        let condition = parse_result.register(self.expr());

        if parse_result.error.is_some() {
            return parse_result;
        }

        if self.current_token_ref().token_type != TokenType::TT_LBRACKET {
            return parse_result.failure(Some(StandardError::new(
                "expected '{'",
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add a '{' to define the body"),
            )));
        }

        parse_result.register_advancement();
        self.advance();

        let body = parse_result.register(self.statements());

        if parse_result.error.is_some() {
            return parse_result;
        }

        if self.current_token_ref().token_type != TokenType::TT_RBRACKET {
            return parse_result.failure(Some(StandardError::new(
                "expected '}'",
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add a '}' to close the body"),
            )));
        }

        parse_result.register_advancement();
        self.advance();

        parse_result.success(Some(Box::new(AstNode::While(WhileNode::new(
            condition.unwrap(),
            body.unwrap(),
        )))))
    }

    fn try_expr(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();

        if !self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "unsafe")
        {
            return parse_result.failure(Some(StandardError::new(
                "expected keyword",
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add the 'unsafe' keyword to represent the block of code that is dangerous"),
            )));
        }

        parse_result.register_advancement();
        self.advance();

        if self.current_token_ref().token_type != TokenType::TT_LBRACKET {
            return parse_result.failure(Some(StandardError::new(
                "expected '{'",
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add a '{' to define the body"),
            )));
        }

        parse_result.register_advancement();
        self.advance();

        let try_body = parse_result.register(self.statements());

        if parse_result.error.is_some() {
            return parse_result;
        }

        if self.current_token_ref().token_type != TokenType::TT_RBRACKET {
            return parse_result.failure(Some(StandardError::new(
                "expected '}'",
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add a '}' to close the body"),
            )));
        }

        parse_result.register_advancement();
        self.advance();

        if !self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "safe")
        {
            return parse_result.failure(Some(StandardError::new(
                "expected keyword",
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add the 'safe' keyword to represent the block of code to fall back to if the 'unsafe' block fails"),
            )));
        }

        parse_result.register_advancement();
        self.advance();

        if self.current_token_ref().token_type != TokenType::TT_IDENTIFIER {
            return parse_result.failure(Some(StandardError::new(
                "expected identifier",
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add a name for the exception error like 'error'"),
            )));
        }

        let error_name_token = self.current_token_copy();

        parse_result.register_advancement();
        self.advance();

        if self.current_token_ref().token_type != TokenType::TT_LBRACKET {
            return parse_result.failure(Some(StandardError::new(
                "expected '{'",
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add a '{' to define the body"),
            )));
        }

        parse_result.register_advancement();
        self.advance();

        let except_body = parse_result.register(self.statements());

        if parse_result.error.is_some() {
            return parse_result;
        }

        if self.current_token_ref().token_type != TokenType::TT_RBRACKET {
            return parse_result.failure(Some(StandardError::new(
                "expected '}'",
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add a '}' to close the body"),
            )));
        }

        parse_result.register_advancement();
        self.advance();

        parse_result.success(Some(Box::new(AstNode::TryExcept(TryExceptNode::new(
            try_body.unwrap(),
            except_body.unwrap(),
            error_name_token,
        )))))
    }

    fn import_expr(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();

        if !self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "fetch")
        {
            return parse_result.failure(Some(StandardError::new(
                "expected keyword",
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add the 'fetch' keyword to import other '.glang' files"),
            )));
        }

        parse_result.register_advancement();
        self.advance();

        let import = parse_result.register(self.expr());

        if parse_result.error.is_some() {
            return parse_result;
        }

        parse_result.register_advancement();
        self.advance();

        parse_result.success(Some(Box::new(AstNode::Import(ImportNode::new(
            import.unwrap(),
        )))))
    }

    fn expr(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let next_tok = match self.next_token_copy() {
            Some(tok) => tok,
            None => Token::new(TokenType::TT_EOF, None, None, None),
        };

        if self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "obj")
        {
            parse_result.register_advancement();
            self.advance();

            if self.current_token_copy().token_type != TokenType::TT_IDENTIFIER {
                return parse_result.failure(Some(StandardError::new(
                    "expected identifier",
                    self.current_pos_start(),
                    self.current_pos_end(),
                    Some("add a name for this object like 'hotdog'"),
                )));
            }

            let var_name = self.current_token_copy();

            parse_result.register_advancement();
            self.advance();

            if self.current_token_copy().token_type != TokenType::TT_EQ {
                return parse_result.failure(Some(StandardError::new(
                    "expected '='",
                    self.current_pos_start(),
                    self.current_pos_end(),
                    Some(
                        format!(
                            "add an '=' to set the value of the variable '{}'",
                            &var_name.value.unwrap()
                        )
                        .as_str(),
                    ),
                )));
            }

            parse_result.register_advancement();
            self.advance();

            let expr = parse_result.register(self.expr());

            if parse_result.error.is_some() {
                return parse_result;
            }

            return parse_result.success(Some(Box::new(AstNode::VariableAssign(
                VariableAssignNode::new(var_name, expr.unwrap()),
            ))));
        } else if self.current_token_copy().token_type == TokenType::TT_IDENTIFIER
            && next_tok.token_type == TokenType::TT_EQ
        {
            let var_name = self.current_token_copy();

            parse_result.register_advancement();
            self.advance();

            // advance past the '=' token too
            parse_result.register_advancement();
            self.advance();

            let expr = parse_result.register(self.expr());

            if parse_result.error.is_some() {
                return parse_result;
            }

            return parse_result.success(Some(Box::new(AstNode::VariableReassign(
                VariableRessignNode::new(var_name, expr.unwrap()),
            ))));
        } else if self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "stay")
        {
            parse_result.register_advancement();
            self.advance();

            if self.current_token_copy().token_type != TokenType::TT_IDENTIFIER {
                return parse_result.failure(Some(StandardError::new(
                    "expected identifier",
                    self.current_pos_start(),
                    self.current_pos_end(),
                    Some("add a name for this constant like 'HOT_DOG'"),
                )));
            }

            let const_name = self.current_token_copy();

            parse_result.register_advancement();
            self.advance();

            if self.current_token_copy().token_type != TokenType::TT_EQ {
                return parse_result.failure(Some(StandardError::new(
                    "expected '='",
                    self.current_pos_start(),
                    self.current_pos_end(),
                    Some(
                        format!(
                            "add an '=' to set the value of the constant '{}'",
                            &const_name.value.unwrap()
                        )
                        .as_str(),
                    ),
                )));
            }

            parse_result.register_advancement();
            self.advance();

            let expr = parse_result.register(self.expr());

            if parse_result.error.is_some() {
                return parse_result;
            }

            return parse_result.success(Some(Box::new(AstNode::ConstAssign(
                ConstAssignNode::new(const_name, expr.unwrap()),
            ))));
        }

        let node = parse_result.register(self.binary_operator(
            "comparison_expr",
            &[
                (TokenType::TT_KEYWORD, "and"),
                (TokenType::TT_KEYWORD, "or"),
            ],
            None,
        ));

        if parse_result.error.is_some() {
            return parse_result.failure(Some(StandardError::new(
                "expected keyword, object, function, expression",
                self.current_pos_start(),
                self.current_pos_end(),
                None,
            )));
        }

        parse_result.success(node)
    }

    fn statement(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let pos_start = self.current_pos_start();

        if self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "give")
        {
            parse_result.register_advancement();
            self.advance();

            let expr = parse_result.try_register(self.expr());

            if expr.is_none() {
                self.reverse(parse_result.to_reverse_count);
            }

            return parse_result.success(Some(Box::new(AstNode::Return(ReturnNode::new(
                expr,
                Some(pos_start),
                Some(self.current_pos_start()),
            )))));
        } else if self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "next")
        {
            parse_result.register_advancement();
            self.advance();

            return parse_result.success(Some(Box::new(AstNode::Continue(ContinueNode::new(
                Some(pos_start),
                Some(self.current_pos_start()),
            )))));
        } else if self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "leave")
        {
            parse_result.register_advancement();
            self.advance();

            return parse_result.success(Some(Box::new(AstNode::Break(BreakNode::new(
                Some(pos_start),
                Some(self.current_pos_start()),
            )))));
        }

        let expr = parse_result.register(self.expr());

        if parse_result.error.is_some() {
            return parse_result.failure(Some(StandardError::new(
                "expected keyword, object, function, expression",
                pos_start,
                self.current_pos_end(),
                None,
            )));
        }

        parse_result.success(expr)
    }

    fn statements(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let mut statements: Vec<Box<AstNode>> = Vec::new();
        let pos_start = self.current_pos_start();

        if self.current_token_ref().token_type == TokenType::TT_EOF {
            return parse_result.success(Some(Box::new(AstNode::List(ListNode::new(
                &[],
                Some(pos_start),
                Some(self.current_pos_end()),
            )))));
        }

        let statement = parse_result.register(self.statement());

        if parse_result.error.is_some() {
            return parse_result;
        }

        statements.push(statement.unwrap());

        loop {
            if self.current_token_ref().token_type == TokenType::TT_SEMICOLON {
                parse_result.register_advancement();
                self.advance();
            }

            match self.current_token_ref().token_type {
                TokenType::TT_EOF | TokenType::TT_RBRACKET => break,
                _ => {}
            }

            let statement = parse_result.register(self.statement());

            if parse_result.error.is_some() {
                return parse_result;
            }

            statements.push(statement.unwrap());
        }

        parse_result.success(Some(Box::new(AstNode::List(ListNode::new(
            &statements,
            Some(pos_start),
            Some(self.current_pos_end()),
        )))))
    }

    fn call(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let atom = parse_result.register(self.atom());

        if parse_result.error.is_some() {
            return parse_result;
        }

        if self.current_token_ref().token_type == TokenType::TT_LPAREN {
            parse_result.register_advancement();
            self.advance();

            let mut arg_nodes: Vec<Box<AstNode>> = Vec::new();

            if self.current_token_ref().token_type == TokenType::TT_RPAREN {
                parse_result.register_advancement();
                self.advance();
            } else {
                let expr = parse_result.register(self.expr());

                if parse_result.error.is_some() {
                    return parse_result.failure(Some(StandardError::new(
                        "expected keyword, object, function, expression",
                        self.current_pos_start(),
                        self.current_pos_end(),
                        None,
                    )));
                }

                arg_nodes.push(expr.unwrap());

                while self.current_token_ref().token_type == TokenType::TT_COMMA {
                    parse_result.register_advancement();
                    self.advance();

                    let expr = parse_result.register(self.expr());

                    if parse_result.error.is_some() {
                        return parse_result;
                    }

                    arg_nodes.push(expr.unwrap());
                }

                if self.current_token_ref().token_type != TokenType::TT_RPAREN {
                    return parse_result.failure(Some(StandardError::new(
                        "expected ',' or ')'",
                        self.current_pos_start(),
                        self.current_pos_end(),
                        Some("add a ',' to input all the function arguments or close with a ')' to call the function"),
                    )));
                }

                parse_result.register_advancement();
                self.advance();
            }

            return parse_result.success(Some(Box::new(AstNode::Call(CallNode::new(
                atom.unwrap().clone(),
                arg_nodes,
            )))));
        }

        parse_result.success(atom)
    }

    fn atom(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let token = self.current_token_copy();

        if [TokenType::TT_INT, TokenType::TT_FLOAT].contains(&token.token_type) {
            parse_result.register_advancement();
            self.advance();

            return parse_result.success(Some(Box::new(AstNode::Number(NumberNode::new(token)))));
        } else if token.token_type == TokenType::TT_STR {
            parse_result.register_advancement();
            self.advance();

            return parse_result.success(Some(Box::new(AstNode::Strings(StringNode::new(token)))));
        } else if token.token_type == TokenType::TT_IDENTIFIER {
            parse_result.register_advancement();
            self.advance();

            return parse_result.success(Some(Box::new(AstNode::VariableAccess(
                VariableAccessNode::new(token),
            ))));
        } else if token.token_type == TokenType::TT_LPAREN {
            parse_result.register_advancement();
            self.advance();
            let expr = parse_result.register(self.expr());

            if parse_result.error.is_some() {
                return parse_result;
            }

            if self.current_token_copy().token_type == TokenType::TT_RPAREN {
                parse_result.register_advancement();
                self.advance();

                return parse_result.success(expr);
            } else {
                return parse_result.failure(Some(StandardError::new(
                    "expected closing parenthesis",
                    self.current_pos_start(),
                    self.current_pos_end(),
                    Some("add a ')' to close the original '('"),
                )));
            }
        } else if token.token_type == TokenType::TT_LSQUARE {
            let expr = parse_result.register(self.list_expr());

            if parse_result.error.is_some() {
                return parse_result;
            }

            return parse_result.success(expr);
        } else if token.matches(TokenType::TT_KEYWORD, "if") {
            let expr = parse_result.register(self.if_expr());

            if parse_result.error.is_some() {
                return parse_result;
            }

            return parse_result.success(expr);
        } else if token.matches(TokenType::TT_KEYWORD, "walk") {
            let expr = parse_result.register(self.for_expr());

            if parse_result.error.is_some() {
                return parse_result;
            }

            return parse_result.success(expr);
        } else if token.matches(TokenType::TT_KEYWORD, "while") {
            let expr = parse_result.register(self.while_expr());

            if parse_result.error.is_some() {
                return parse_result;
            }

            return parse_result.success(expr);
        } else if token.matches(TokenType::TT_KEYWORD, "unsafe") {
            let expr = parse_result.register(self.try_expr());

            if parse_result.error.is_some() {
                return parse_result;
            }

            return parse_result.success(expr);
        } else if token.matches(TokenType::TT_KEYWORD, "func") {
            let func_def = parse_result.register(self.func_definition());

            if parse_result.error.is_some() {
                return parse_result;
            }

            return parse_result.success(func_def);
        } else if token.matches(TokenType::TT_KEYWORD, "fetch") {
            let import_expr = parse_result.register(self.import_expr());

            if parse_result.error.is_some() {
                return parse_result;
            }

            return parse_result.success(import_expr);
        }

        parse_result.failure(Some(StandardError::new(
            "expected object, keyword, function, or expression",
            Rc::new(token.pos_start.unwrap()),
            Rc::new(token.pos_end.unwrap()),
            None,
        )))
    }

    fn power(&mut self) -> ParseResult {
        self.binary_operator("call", &[(TokenType::TT_POW, "")], Some("factor"))
    }

    fn factor(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let token = self.current_token_copy();

        if [TokenType::TT_PLUS, TokenType::TT_MINUS].contains(&token.token_type) {
            parse_result.register_advancement();
            self.advance();
            let factor = parse_result.register(self.factor());

            if parse_result.error.is_some() {
                return parse_result;
            }

            return parse_result.success(Some(Box::new(AstNode::UnaryOperator(
                UnaryOperatorNode::new(token, factor.unwrap()),
            ))));
        }

        self.power()
    }

    fn term(&mut self) -> ParseResult {
        self.binary_operator(
            "factor",
            &[
                (TokenType::TT_MUL, ""),
                (TokenType::TT_DIV, ""),
                (TokenType::TT_MOD, ""),
            ],
            None,
        )
    }

    fn func_definition(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();

        if !self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "func")
        {
            return parse_result.failure(Some(StandardError::new(
                "expected keyword",
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add the 'func' keyword to define a function"),
            )));
        }

        parse_result.register_advancement();
        self.advance();

        let var_name_token: Option<Token>;

        if self.current_token_ref().token_type == TokenType::TT_IDENTIFIER {
            var_name_token = Some(self.current_token_copy());
            parse_result.register_advancement();
            self.advance();

            if self.current_token_ref().token_type != TokenType::TT_LPAREN {
                return parse_result.failure(Some(StandardError::new(
                    "expected '('",
                    self.current_pos_start(),
                    self.current_pos_end(),
                    Some("add a '(' to define the function arguments"),
                )));
            }
        } else {
            var_name_token = None;

            if self.current_token_ref().token_type != TokenType::TT_LPAREN {
                return parse_result.failure(Some(StandardError::new(
                    "expected identifier or '('",
                    self.current_pos_start(),
                    self.current_pos_end(),
                    Some("add a name for this function like 'greet' or use '(' to define an anonymous function"),
                )));
            }
        }

        parse_result.register_advancement();
        self.advance();

        let mut arg_name_tokens: Vec<Token> = Vec::new();

        if self.current_token_ref().token_type == TokenType::TT_IDENTIFIER {
            arg_name_tokens.push(self.current_token_copy());

            parse_result.register_advancement();
            self.advance();

            while self.current_token_ref().token_type == TokenType::TT_COMMA {
                parse_result.register_advancement();
                self.advance();

                if self.current_token_ref().token_type == TokenType::TT_RPAREN {
                    break;
                }

                if self.current_token_ref().token_type != TokenType::TT_IDENTIFIER {
                    return parse_result.failure(Some(StandardError::new(
                        "expected identifier",
                        self.current_pos_start(),
                        self.current_pos_end(),
                        Some("add a name for the function arguments like 'name'"),
                    )));
                }

                arg_name_tokens.push(self.current_token_copy());

                parse_result.register_advancement();
                self.advance();
            }

            if self.current_token_ref().token_type != TokenType::TT_RPAREN {
                return parse_result.failure(Some(StandardError::new(
                    "expected ')'",
                    self.current_pos_start(),
                    self.current_pos_end(),
                    Some("add another function argument or complete the function with ')'"),
                )));
            }
        } else if self.current_token_ref().token_type != TokenType::TT_RPAREN {
            return parse_result.failure(Some(StandardError::new(
                "expected indentifier or ')'",
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add a name for the function arguments like 'name' or complete the function with ')'"),
            )));
        }

        parse_result.register_advancement();
        self.advance();

        if self.current_token_ref().token_type != TokenType::TT_LBRACKET {
            return parse_result.failure(Some(StandardError::new(
                "expected '{'",
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add a '{' to define the body of the function"),
            )));
        }

        parse_result.register_advancement();
        self.advance();

        let body = parse_result.register(self.statements());

        if parse_result.error.is_some() {
            return parse_result;
        }

        if self.current_token_ref().token_type != TokenType::TT_RBRACKET {
            return parse_result.failure(Some(StandardError::new(
                "expected '}'",
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add a '}' to close the body"),
            )));
        }

        parse_result.register_advancement();
        self.advance();

        parse_result.success(Some(Box::new(AstNode::FunctionDefinition(
            FunctionDefinitionNode::new(var_name_token, &arg_name_tokens, body.unwrap(), false),
        ))))
    }

    fn binary_operator(
        &mut self,
        func_a: &str,
        ops: &[(TokenType, &str)],
        func_b: Option<&str>,
    ) -> ParseResult {
        let func_b = func_b.unwrap_or(func_a);

        let mut parse_result = ParseResult::new();
        let mut left = parse_result.register(match func_a {
            "comparison_expr" => self.comparison_expr(),
            "arithmetic_expr" => self.arithmetic_expr(),
            "term" => self.term(),
            "factor" => self.factor(),
            "call" => self.call(),
            _ => panic!("CRITICAL ERROR: GLANG COULD NOT FIND EXPRESSION IN BINARY OPERATOR"),
        });

        if parse_result.error.is_some() {
            return parse_result;
        }

        while ops.contains(&(
            self.current_token.clone().unwrap().token_type,
            self.current_token
                .clone()
                .unwrap()
                .value
                .unwrap_or_default()
                .as_str(),
        )) || ops.contains(&(self.current_token.clone().unwrap().token_type, ""))
        {
            let op_token = self.current_token.clone().unwrap().clone();
            parse_result.register_advancement();
            self.advance();
            let right = parse_result.register(match func_b {
                "comparison_expr" => self.comparison_expr(),
                "arithmetic_expr" => self.arithmetic_expr(),
                "term" => self.term(),
                "factor" => self.factor(),
                "call" => self.call(),
                _ => panic!("CRITICAL ERROR: GLANG COULD NOT FIND EXPRESSION IN BINARY OPERATOR"),
            });

            if parse_result.error.is_some() {
                return parse_result;
            }

            left = Some(Box::new(AstNode::BinaryOperator(BinaryOperatorNode::new(
                left.unwrap().clone(),
                op_token,
                right.unwrap(),
            ))));
        }

        parse_result.success(left)
    }
}

// Test the output AST from parsed tokens
#[test]
fn test_ast() {
    use glang_lexer::Lexer;

    let mut lexer = Lexer::new("<test>", "function(1 + 1);".to_owned());
    let tokens = lexer.make_tokens().ok().unwrap();

    let mut parser = Parser::new(&tokens);
    let ast = parser.parse();

    let node = match *ast.node.unwrap() {
        AstNode::List(l) => l,
        _ => panic!("Expected a list node"),
    };

    assert_eq!(node.element_nodes.len(), 1); // only one call node
    assert!(matches!(*node.element_nodes[0], AstNode::Call { .. }));
}
