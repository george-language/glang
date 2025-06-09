use crate::{
    errors::standard_error::StandardError,
    lexing::{position::Position, token::Token, token_type::TokenType},
    nodes::{
        binary_operator_node::BinaryOperatorNode, common_node::CommonNode, number_node::NumberNode,
        return_node::ReturnNode, string_node::StringNode, unary_operator_node::UnaryOperatorNode,
        variable_assign_node::VariableAssignNode,
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

    pub fn update_current_token(&mut self) {
        if self.token_index >= 0 && self.token_index < self.tokens.len() as isize {
            self.current_token = Some(self.tokens[self.token_index as usize].clone());
        }
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

        if parse_result.error.is_some()
            && self.current_token.as_ref().unwrap().token_type != TokenType::TT_EOF
        {
            return parse_result.failure(Some(StandardError::new(
                "expected operator or bracket".to_string(),
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add one of the following: '+', '-', '*', '/', or '}'".to_string()),
            )));
        }

        parse_result
    }

    pub fn comparison_expr(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();

        if self
            .current_token
            .as_ref()
            .unwrap()
            .matches(TokenType::TT_KEYWORD, Some("oppositeof"))
        {
            let op_token = self.current_token.as_ref().unwrap().clone();
            parse_result.register_advancement();
            self.advance();

            let node = parse_result
                .register(self.comparison_expr())
                .unwrap()
                .clone();

            if parse_result.error.is_some() {
                return parse_result;
            }

            return parse_result.success(Some(Box::new(UnaryOperatorNode::new(
                op_token.clone(),
                node.clone(),
            )) as Box<dyn CommonNode>));
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
                "expected an object or operator".to_string(),
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add one of the following: integer, float, identifier, 'oppositeof', '+', '-', '(', or '['".to_string()),
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

    pub fn expr(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();

        if self
            .current_token
            .as_ref()
            .unwrap()
            .matches(TokenType::TT_KEYWORD, Some("obj"))
        {
            parse_result.register_advancement();
            self.advance();

            if self.current_token.as_ref().unwrap().token_type != TokenType::TT_IDENTIFIER {
                return parse_result.failure(Some(StandardError::new(
                    "expected identifier".to_string(),
                    self.current_pos_start(),
                    self.current_pos_end(),
                    Some("add a name for this object like 'hotdog'".to_string()),
                )));
            }

            let var_name = self.current_token.clone().unwrap();
            parse_result.register_advancement();
            self.advance();

            if self.current_token.as_ref().unwrap().token_type != TokenType::TT_EQ {
                return parse_result.failure(Some(StandardError::new(
                    "expected '='".to_string(),
                    self.current_pos_start(),
                    self.current_pos_end(),
                    Some(
                        format!(
                            "add a '=' to set the value of the variable '{}'",
                            var_name.value.unwrap().clone()
                        )
                        .to_string(),
                    ),
                )));
            }

            parse_result.register_advancement();
            self.advance();
            let expr = parse_result.register(self.expr());

            if parse_result.error.is_some() {
                return parse_result;
            }

            return parse_result.success(Some(Box::new(VariableAssignNode::new(
                var_name.clone(),
                expr.unwrap(),
            )) as Box<dyn CommonNode>));
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
                "expected a keyword, type, or function".to_string(),
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add one of the following: 'obj', 'if', 'walk', 'while', 'func', 'oppositeof', integer, float, identifier, '+', '-', '(', or '['".to_string()),
            )));
        }

        parse_result.success(node)
    }

    pub fn statement(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let pos_start = self.current_pos_start();

        if self
            .current_token
            .as_ref()
            .unwrap()
            .matches(TokenType::TT_KEYWORD, Some("give"))
        {
            parse_result.register_advancement();
            self.advance();

            let expr = parse_result.try_register(self.expr());

            if expr.is_none() {
                self.reverse(parse_result.to_reverse_count);
            }

            return parse_result.success(Some(Box::new(ReturnNode::new(
                expr.unwrap(),
                Some(pos_start),
                Some(self.current_pos_start()),
            )) as Box<dyn CommonNode>));
        }

        let expr = parse_result.register(self.expr());

        if parse_result.error.is_some() {
            return parse_result.failure(Some(StandardError::new(
                "expected keyword, object, or operator".to_string(),
                self.current_pos_start(),
                self.current_pos_end(),
                Some("add any of the following: 'give', 'next', 'leave', 'obj', 'oppositeof', 'if', 'walk', 'while', 'func', int, float, identifier, '+', '-', '(', or '['".to_string()),
            )));
        }

        parse_result.success(expr)
    }

    pub fn statements(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let statements: Vec<ParseResult> = Vec::new();
        let pos_start = self.current_pos_start();

        while self.current_token.as_mut().unwrap().token_type == TokenType::TT_NEWLINE {
            parse_result.register_advancement();
            self.advance();
        }

        let statement = parse_result.register(self.statement());

        parse_result
    }

    pub fn call(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let atom = parse_result.register(self.atom());

        if parse_result.error.is_some() {
            return parse_result;
        }

        parse_result.success(atom)
    }

    pub fn atom(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let token = self.current_token.as_ref().unwrap().clone();

        if [TokenType::TT_INT, TokenType::TT_FLOAT].contains(&token.token_type) {
            parse_result.register_advancement();
            self.advance();

            return parse_result
                .success(Some(Box::new(NumberNode::new(token)) as Box<dyn CommonNode>));
        } else if token.token_type == TokenType::TT_STR {
            parse_result.register_advancement();
            self.advance();

            return parse_result
                .success(Some(Box::new(StringNode::new(token)) as Box<dyn CommonNode>));
        }

        parse_result.failure(Some(StandardError::new(
            "expected object, keyword, function, or type".to_string(),
            token.pos_start.unwrap(),
            token.pos_end.unwrap(),
            Some("add any of the following: 'obj', 'if', 'walk', 'while', 'func', integer, float, identifier, '+' , '-' , '(' or '['".to_string()),
        )))
    }

    pub fn power(&mut self) -> ParseResult {
        self.binary_operator("call", vec![(TokenType::TT_POW, "")], Some("factor"))
    }

    pub fn factor(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let token = self.current_token.as_ref().unwrap().clone();

        if [TokenType::TT_PLUS, TokenType::TT_MINUS].contains(&token.token_type) {
            parse_result.register_advancement();
            self.advance();
            let factor = parse_result.register(self.factor());

            if parse_result.error.is_some() {
                return parse_result;
            }

            return parse_result.success(Some(Box::new(UnaryOperatorNode::new(
                token,
                factor.unwrap(),
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

            left = Some(Box::new(BinaryOperatorNode::new(
                left.unwrap().clone(),
                op_token,
                right.unwrap(),
            )) as Box<dyn CommonNode>);
        }

        parse_result.success(left)
    }
}
