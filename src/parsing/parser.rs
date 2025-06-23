use crate::{
    errors::standard_error::StandardError,
    lexing::{position::Position, token::Token, token_type::TokenType},
    nodes::{
        ast_node::AstNode, binary_operator_node::BinaryOperatorNode, break_node::BreakNode,
        call_node::CallNode, continue_node::ContinueNode, for_node::ForNode,
        function_definition_node::FunctionDefinitionNode, if_node::IfNode, list_node::ListNode,
        number_node::NumberNode, return_node::ReturnNode, string_node::StringNode,
        unary_operator_node::UnaryOperatorNode, variable_access_node::VariableAccessNode,
        variable_assign_node::VariableAssignNode, while_node::WhileNode,
    },
    parsing::parse_result::ParseResult,
};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub token_index: isize,
    pub current_token: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Parser {
            tokens: tokens,
            token_index: -1,
            current_token: None,
        };
        parser.advance();

        parser
    }

    pub fn advance(&mut self) -> Option<Token> {
        self.token_index += 1;
        self.update_current_token();

        self.current_token.clone()
    }

    pub fn reverse(&mut self, amount: usize) -> Option<Token> {
        self.token_index -= amount as isize;
        self.update_current_token();

        self.current_token.clone()
    }

    pub fn skip_newlines(&mut self, parse_result: &mut ParseResult) {
        while self.current_token_ref().token_type == TokenType::TT_NEWLINE {
            parse_result.register_advancement();
            self.advance();
        }
    }

    pub fn update_current_token(&mut self) {
        if self.token_index >= 0 && self.token_index < self.tokens.len() as isize {
            self.current_token = Some(self.tokens[self.token_index as usize].clone());
        }
    }

    pub fn current_token_copy(&mut self) -> Token {
        self.current_token.as_ref().unwrap().clone()
    }

    pub fn current_token_ref(&mut self) -> &Token {
        &self.current_token.as_ref().unwrap()
    }

    pub fn current_pos_start(&self) -> Position {
        self.current_token
            .as_ref()
            .and_then(|tok| tok.pos_start.as_ref())
            .cloned()
            .expect("Expected a pos_start")
    }

    pub fn current_pos_end(&self) -> Position {
        self.current_token
            .as_ref()
            .and_then(|tok| tok.pos_end.as_ref())
            .cloned()
            .expect("Expected a pos_end")
    }

    pub fn parse(&mut self) -> ParseResult {
        let mut parse_result = self.statements();

        if parse_result.error.is_some() && self.current_token_copy().token_type != TokenType::TT_EOF
        {
            return parse_result.failure(Some(StandardError::new(
                "expected operator or bracket",
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add one of the following: '+', '-', '*', '/', or '}'"),
            )));
        }

        parse_result
    }

    pub fn comparison_expr(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();

        if self
            .current_token_copy()
            .matches(TokenType::TT_KEYWORD, Some("oppositeof"))
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
            vec![
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
                Some("add one of the following: integer, float, identifier, 'oppositeof', '+', '-', '(', or '['"),
            )));
        }

        parse_result.success(node)
    }

    pub fn arithmetic_expr(&mut self) -> ParseResult {
        self.binary_operator(
            "term",
            vec![(TokenType::TT_PLUS, ""), (TokenType::TT_MINUS, "")],
            None,
        )
    }

    pub fn list_expr(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let mut element_nodes: Vec<Option<Box<AstNode>>> = Vec::new();
        let pos_start = self.current_token_ref().pos_start.clone();

        if self.current_token_ref().token_type != TokenType::TT_LSQUARE {
            return parse_result.failure(Some(StandardError::new(
                "expected list initializing bracket",
                self.current_token_copy().pos_start.unwrap(),
                self.current_token_copy().pos_end.unwrap(),
                Some("add a '[' to start the list"),
            )));
        }

        parse_result.register_advancement();
        self.advance();

        if self.current_token_ref().token_type == TokenType::TT_RSQUARE {
            parse_result.register_advancement();
            self.advance();
        } else {
            element_nodes.push(parse_result.register(self.expr()));

            if parse_result.error.is_some() {
                return parse_result.failure(Some(StandardError::new(
                    "expected closing bracket or list element",
                    self.current_token_copy().pos_start.unwrap(),
                    self.current_token_copy().pos_end.unwrap(),
                    Some("add a ']' to close the list or add a list element followed by a comma"),
                )));
            }

            while self.current_token_ref().token_type == TokenType::TT_COMMA {
                parse_result.register_advancement();
                self.advance();

                element_nodes.push(parse_result.register(self.expr()));

                if parse_result.error.is_some() {
                    return parse_result;
                }
            }

            if self.current_token_ref().token_type != TokenType::TT_RSQUARE {
                return parse_result.failure(Some(StandardError::new(
                    "expected closing bracket or next list element",
                    self.current_token_copy().pos_start.unwrap(),
                    self.current_token_copy().pos_end.unwrap(),
                    Some("add a ']' to close the list or add a list element followed by a comma"),
                )));
            }

            parse_result.register_advancement();
            self.advance();
        }

        parse_result.success(Some(Box::new(AstNode::List(ListNode::new(
            element_nodes.clone(),
            pos_start,
            self.current_token_copy().pos_end,
        )))))
    }

    pub fn if_expr(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let (if_parse_result, cases, else_case) = self.if_expr_cases("if");

        if if_parse_result.error.is_some() {
            return if_parse_result;
        }

        parse_result.success(Some(Box::new(AstNode::If(IfNode::new(cases, else_case)))))
    }

    pub fn if_expr_b(
        &mut self,
    ) -> (
        ParseResult,
        Vec<(Box<AstNode>, Box<AstNode>, bool)>,
        Option<(Box<AstNode>, bool)>,
    ) {
        self.if_expr_cases("alsoif")
    }

    pub fn if_expr_c(&mut self) -> (ParseResult, Option<(Box<AstNode>, bool)>) {
        let mut parse_result = ParseResult::new();
        let mut else_case: Option<(Box<AstNode>, bool)> = None;

        if self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, Some("otherwise"))
        {
            parse_result.register_advancement();
            self.advance();

            self.skip_newlines(&mut parse_result);

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

    pub fn if_expr_b_or_c(
        &mut self,
    ) -> (
        ParseResult,
        Vec<(Box<AstNode>, Box<AstNode>, bool)>,
        Option<(Box<AstNode>, bool)>,
    ) {
        let mut parse_result = ParseResult::new();
        let mut cases: Vec<(Box<AstNode>, Box<AstNode>, bool)> = Vec::new();
        let mut else_case: Option<(Box<AstNode>, bool)> = None;

        if self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, Some("alsoif"))
        {
            parse_result.register_advancement();
            self.advance();

            let (if_parse_result, all_cases, else_clause) = self.if_expr_b();

            if if_parse_result.error.is_some() {
                return (if_parse_result, Vec::new(), None);
            }

            else_case = else_clause;
            cases = all_cases.clone();
        } else {
            let (if_parse_result, else_clause) = self.if_expr_c();

            if if_parse_result.error.is_some() {
                return (if_parse_result, Vec::new(), None);
            }

            else_case = else_clause;
        }

        (parse_result, cases, else_case)
    }

    pub fn if_expr_cases(
        &mut self,
        keyword: &'static str,
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
            .matches(TokenType::TT_KEYWORD, Some(keyword))
        {
            return (
                parse_result.failure(Some(StandardError::new(
                    "expected keyword",
                    self.current_pos_start(),
                    self.current_pos_end(),
                    Some(format!("add the '{}' keyword", keyword).as_str()),
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

        self.skip_newlines(&mut parse_result);

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

        cases.push((
            condition.unwrap().clone(),
            statements.unwrap().clone(),
            true,
        ));

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

        return (parse_result, cases, else_case);
    }

    pub fn for_expr(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();

        if !self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, Some("walk"))
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
            .matches(TokenType::TT_KEYWORD, Some("through"))
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
            .matches(TokenType::TT_KEYWORD, Some("step"))
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

        self.skip_newlines(&mut parse_result);

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

        if self.current_token_ref().token_type == TokenType::TT_NEWLINE {
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

            return parse_result.success(Some(Box::new(AstNode::For(ForNode::new(
                var_name,
                start_value.unwrap(),
                end_value.unwrap(),
                step_value,
                body.unwrap(),
                true,
            )))));
        }

        let body = parse_result.register(self.statement());

        if parse_result.error.is_some() {
            return parse_result;
        }

        parse_result.success(Some(Box::new(AstNode::For(ForNode::new(
            var_name,
            start_value.unwrap(),
            end_value.unwrap(),
            step_value,
            body.unwrap(),
            false,
        )))))
    }

    pub fn while_expr(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();

        if !self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, Some("while"))
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

        self.skip_newlines(&mut parse_result);

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

        return parse_result.success(Some(Box::new(AstNode::While(WhileNode::new(
            condition.unwrap(),
            body.unwrap(),
            true,
        )))));
    }

    pub fn expr(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();

        if self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, Some("obj"))
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
                            var_name.value.unwrap().clone()
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
                VariableAssignNode::new(var_name.clone(), expr.unwrap()),
            ))));
        }

        let node = parse_result.register(self.binary_operator(
            "comparison_expr",
            vec![
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

    pub fn statement(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let pos_start = self.current_pos_start();

        if self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, Some("give"))
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
            .matches(TokenType::TT_KEYWORD, Some("next"))
        {
            parse_result.register_advancement();
            self.advance();

            return parse_result.success(Some(Box::new(AstNode::Continue(ContinueNode::new(
                Some(pos_start),
                Some(self.current_pos_start()),
            )))));
        } else if self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, Some("leave"))
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

    pub fn statements(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let mut statements: Vec<Option<Box<AstNode>>> = Vec::new();
        let pos_start = self.current_pos_start();

        while self.current_token_ref().token_type == TokenType::TT_NEWLINE {
            parse_result.register_advancement();
            self.advance();
        }

        if self.current_token_ref().token_type == TokenType::TT_EOF {
            return parse_result.success(Some(Box::new(AstNode::List(ListNode::new(
                Vec::new(),
                Some(pos_start),
                Some(self.current_pos_end()),
            )))));
        }

        let statement = parse_result.register(self.statement());

        if parse_result.error.is_some() {
            return parse_result;
        }

        statements.push(statement);

        let mut more_statements = true;

        loop {
            let mut newline_count: usize = 0;

            while self.current_token_ref().token_type == TokenType::TT_NEWLINE {
                parse_result.register_advancement();
                self.advance();

                newline_count += 1;
            }

            if newline_count == 0 {
                more_statements = false;
            }

            if !more_statements {
                break;
            }

            if self.current_token_ref().token_type == TokenType::TT_EOF {
                break;
            }

            if self.current_token_ref().token_type == TokenType::TT_RBRACKET {
                break;
            }

            let statement = parse_result.register(self.statement());

            if parse_result.error.is_some() {
                return parse_result;
            }

            statements.push(statement);
        }

        return parse_result.success(Some(Box::new(AstNode::List(ListNode::new(
            statements,
            Some(pos_start),
            Some(self.current_pos_end()),
        )))));
    }

    pub fn call(&mut self) -> ParseResult {
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

                    arg_nodes.push(parse_result.register(self.expr()).unwrap());

                    if parse_result.error.is_some() {
                        return parse_result;
                    }
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

    pub fn atom(&mut self) -> ParseResult {
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
        } else if token.matches(TokenType::TT_KEYWORD, Some("if")) {
            let expr = parse_result.register(self.if_expr());

            if parse_result.error.is_some() {
                return parse_result;
            }

            return parse_result.success(expr);
        } else if token.matches(TokenType::TT_KEYWORD, Some("walk")) {
            let expr = parse_result.register(self.for_expr());

            if parse_result.error.is_some() {
                return parse_result;
            }

            return parse_result.success(expr);
        } else if token.matches(TokenType::TT_KEYWORD, Some("while")) {
            let expr = parse_result.register(self.while_expr());

            if parse_result.error.is_some() {
                return parse_result;
            }

            return parse_result.success(expr);
        } else if token.matches(TokenType::TT_KEYWORD, Some("func")) {
            let func_def = parse_result.register(self.func_definition());

            if parse_result.error.is_some() {
                return parse_result;
            }

            return parse_result.success(func_def);
        }

        parse_result.failure(Some(StandardError::new(
            "expected object, keyword, function, or expression",
            token.pos_start.unwrap(),
            token.pos_end.unwrap(),
            None,
        )))
    }

    pub fn power(&mut self) -> ParseResult {
        self.binary_operator("call", vec![(TokenType::TT_POW, "")], Some("factor"))
    }

    pub fn factor(&mut self) -> ParseResult {
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

        return self.power();
    }

    pub fn term(&mut self) -> ParseResult {
        self.binary_operator(
            "factor",
            vec![(TokenType::TT_MUL, ""), (TokenType::TT_DIV, "")],
            None,
        )
    }

    pub fn func_definition(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();

        if !self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, Some("func"))
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
                    "expected comma or ')'",
                    self.current_pos_start(),
                    self.current_pos_end(),
                    Some("add a ',' followed by the function argument or complete the function with ')'"),
                )));
            }
        } else {
            if self.current_token_ref().token_type != TokenType::TT_RPAREN {
                return parse_result.failure(Some(StandardError::new(
                    "expected indentifier or ')'",
                    self.current_pos_start(),
                    self.current_pos_end(),
                    Some("add a name for the function arguments like 'name' or complete the function with ')'"),
                )));
            }
        }

        parse_result.register_advancement();
        self.advance();

        self.skip_newlines(&mut parse_result);

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
            FunctionDefinitionNode::new(var_name_token, arg_name_tokens, body.unwrap(), false),
        ))))
    }

    pub fn binary_operator(
        &mut self,
        func_a: &'static str,
        ops: Vec<(TokenType, &'static str)>,
        func_b: Option<&'static str>,
    ) -> ParseResult {
        let func_b = func_b.unwrap_or_else(|| func_a);

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
                .unwrap_or_else(|| "".to_string())
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
