use crate::{
    ParseResult,
    ast_node::{AstArena, NodeID},
};
use glang_attributes::{Position, Span, StandardError};
use glang_lexer::{Token, TokenType};
use std::{rc::Rc, time::Instant};

pub fn parse(tokens: &[Token], contents: &str) -> Result<AstArena, StandardError> {
    let parsing_time = Instant::now();

    let mut parser = Parser::new(tokens, contents);
    let ast = parser.parse();

    if let Some(e) = ast.error {
        return Err(e);
    }

    if cfg!(feature = "benchmark") {
        println!("Time to parse: {:?}ms", parsing_time.elapsed().as_millis())
    }

    Ok(parser.arena)
}

#[derive(Debug, Clone)]
enum Operator {
    ComparisonExpr,
    ArithmeticExpr,
    Term,
    Factor,
    Call,
}

pub struct Parser {
    pub tokens: Rc<[Token]>,
    pub token_index: isize,
    pub current_token: Option<Token>,
    pub arena: AstArena,
    contents: String,
}

impl Parser {
    pub fn new(tokens: &[Token], contents: &str) -> Self {
        let mut parser = Self {
            tokens: Rc::from(tokens),
            token_index: -1,
            current_token: None,
            arena: AstArena::new(),
            contents: contents.to_owned(),
        };
        parser.advance();

        parser
    }

    pub fn contents(&self) -> &str {
        &self.contents
    }

    pub fn parse(&mut self) -> ParseResult {
        let mut parse_result = self.statements();

        if parse_result.error.is_some() && self.current_token_copy().token_type != TokenType::TT_EOF
        {
            return parse_result.failure(StandardError::new(
                "expected keyword, object, function, expression",
                self.current_span(),
                None,
            ));
        }

        parse_result
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

    fn current_token_ref(&mut self) -> &Token {
        self.current_token.as_ref().unwrap()
    }

    fn next_token_copy(&mut self) -> Option<Token> {
        if self.token_index >= 0 && self.token_index + 1 < self.tokens.len() as isize {
            return Some(self.tokens[self.token_index as usize + 1].clone());
        }

        None
    }

    fn current_span(&self) -> Span {
        self.current_token.as_ref().unwrap().span.clone()
    }

    fn current_position_start(&self) -> Position {
        self.current_token.as_ref().unwrap().span.start.clone()
    }

    fn current_position_end(&self) -> Position {
        self.current_token.as_ref().unwrap().span.end.clone()
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

            let node = parse_result.register(self.comparison_expr()).clone();

            if parse_result.error.is_some() {
                return parse_result;
            }

            return parse_result.success(
                self.arena
                    .unary_operator_node(op_token.clone(), node.clone()),
            );
        }

        let node = parse_result.register(self.binary_operator(
            Operator::ArithmeticExpr,
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
            return parse_result.failure(StandardError::new(
                "expected keyword, object, function, expression",
                self.current_span(),
                None,
            ));
        }

        parse_result.success(node)
    }

    fn arithmetic_expr(&mut self) -> ParseResult {
        self.binary_operator(
            Operator::Term,
            &[(TokenType::TT_PLUS, ""), (TokenType::TT_MINUS, "")],
            None,
        )
    }

    fn list_expr(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let mut element_nodes: Vec<NodeID> = Vec::new();
        let pos_start = self.current_position_start();

        if self.current_token_ref().token_type != TokenType::TT_LSQUARE {
            return parse_result.failure(StandardError::new(
                "expected list initializing bracket",
                self.current_span(),
                Some("add a '[' to start the list"),
            ));
        }

        parse_result.register_advancement();
        self.advance();

        if self.current_token_ref().token_type == TokenType::TT_RSQUARE {
            parse_result.register_advancement();
            self.advance();
        } else {
            let element = parse_result.register(self.expr());

            if parse_result.error.is_some() {
                return parse_result.failure(StandardError::new(
                    "expected closing bracket or list element",
                    self.current_span(),
                    Some("add a ']' to close the list or add a list element followed by a comma"),
                ));
            }

            element_nodes.push(element);

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

                element_nodes.push(element);
            }

            if self.current_token_ref().token_type != TokenType::TT_RSQUARE {
                return parse_result.failure(StandardError::new(
                    "expected ']' or next list element",
                    self.current_span(),
                    Some("add a ']' to close the list or add a list element followed by a comma"),
                ));
            }

            parse_result.register_advancement();
            self.advance();
        }

        parse_result.success(self.arena.list_node(
            element_nodes,
            Span::new(
                &self.current_span().filename,
                pos_start.clone(),
                self.current_position_end().clone(),
            ),
        ))
    }

    fn if_expr(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let (if_parse_result, cases, else_case) = self.if_expr_cases("if");

        if if_parse_result.error.is_some() {
            return if_parse_result;
        }

        parse_result.success(self.arena.if_node(cases, else_case))
    }

    fn if_expr_b(
        &mut self,
    ) -> (
        ParseResult,
        Vec<(NodeID, NodeID, bool)>,
        Option<(NodeID, bool)>,
    ) {
        self.if_expr_cases("also")
    }

    fn if_expr_c(&mut self) -> (ParseResult, Option<(NodeID, bool)>) {
        let mut parse_result = ParseResult::new();
        let mut else_case: Option<(NodeID, bool)> = None;

        if self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "otherwise")
        {
            parse_result.register_advancement();
            self.advance();

            if self.current_token_ref().token_type != TokenType::TT_LBRACKET {
                return (
                    parse_result.failure(StandardError::new(
                        "expected '{'",
                        self.current_span(),
                        Some("add a '{' to define the body"),
                    )),
                    None,
                );
            }

            parse_result.register_advancement();
            self.advance();

            let statements = parse_result.register(self.statements());

            if parse_result.error.is_some() {
                return (parse_result, None);
            }

            let body = (statements, true);

            else_case = Some(body);

            if self.current_token_ref().token_type != TokenType::TT_RBRACKET {
                return (
                    parse_result.failure(StandardError::new(
                        "expected '}'",
                        self.current_span(),
                        Some("add a '}' to close the body"),
                    )),
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
        Vec<(NodeID, NodeID, bool)>,
        Option<(NodeID, bool)>,
    ) {
        let mut parse_result = ParseResult::new();
        let mut cases: Vec<(NodeID, NodeID, bool)> = Vec::new();
        let mut else_case: Option<(NodeID, bool)> = None;

        while self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "also")
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
        Vec<(NodeID, NodeID, bool)>,
        Option<(NodeID, bool)>,
    ) {
        let mut parse_result = ParseResult::new();
        let mut cases: Vec<(NodeID, NodeID, bool)> = Vec::new();
        let else_case: Option<(NodeID, bool)>;

        if !self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, keyword)
        {
            return (
                parse_result.failure(StandardError::new(
                    "expected keyword",
                    self.current_span(),
                    Some(format!("add the '{keyword}' keyword").as_str()),
                )),
                Vec::new(),
                None,
            );
        }

        parse_result.register_advancement();
        self.advance();

        if keyword == "also" {
            if !self
                .current_token_ref()
                .matches(TokenType::TT_KEYWORD, "if")
            {
                return (
                    parse_result.failure(StandardError::new(
                        "expected 'if' after 'also'",
                        self.current_span(),
                        Some(format!("add the 'if' keyword").as_str()),
                    )),
                    Vec::new(),
                    None,
                );
            }

            parse_result.register_advancement();
            self.advance();
        }

        let condition = parse_result.register(self.statement());

        if parse_result.error.is_some() {
            return (parse_result, Vec::new(), None);
        }

        if self.current_token_ref().token_type != TokenType::TT_LBRACKET {
            return (
                parse_result.failure(StandardError::new(
                    "expected '{'",
                    self.current_span(),
                    Some("add a '{' to define the body"),
                )),
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

        cases.push((condition, statements, true));

        if self.current_token_ref().token_type != TokenType::TT_RBRACKET {
            return (
                parse_result.failure(StandardError::new(
                    "expected '}'",
                    self.current_span(),
                    Some("add a '}' to close the body"),
                )),
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
            return parse_result.failure(StandardError::new(
                "expected keyword",
                self.current_span(),
                Some("add the 'walk' keyword to initialize a walk loop"),
            ));
        }

        parse_result.register_advancement();
        self.advance();

        if self.current_token_ref().token_type != TokenType::TT_IDENTIFIER {
            return parse_result.failure(StandardError::new(
                "expected identifier",
                self.current_span(),
                Some("add an object name like 'i' for the iterator name"),
            ));
        }

        let var_name = self.current_token_copy();

        parse_result.register_advancement();
        self.advance();

        if self.current_token_ref().token_type == TokenType::TT_EQ {
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
                return parse_result.failure(StandardError::new(
                    "expected 'through'",
                    self.current_span(),
                    Some("add the 'through' keyword to define a range 'n through n'"),
                ));
            }

            parse_result.register_advancement();
            self.advance();

            let end_value = parse_result.register(self.expr());

            if parse_result.error.is_some() {
                return parse_result;
            }

            let step_value: Option<NodeID>;

            if self
                .current_token_ref()
                .matches(TokenType::TT_KEYWORD, "step")
            {
                parse_result.register_advancement();
                self.advance();

                if self.current_token_ref().token_type != TokenType::TT_EQ {
                    return parse_result.failure(StandardError::new(
                        "expected '='",
                        self.current_span(),
                        Some("add an '=' to set the step amount"),
                    ));
                }

                parse_result.register_advancement();
                self.advance();

                step_value = Some(parse_result.register(self.expr()));

                if parse_result.error.is_some() {
                    return parse_result;
                }
            } else {
                step_value = None;
            }

            if self.current_token_ref().token_type != TokenType::TT_LBRACKET {
                return parse_result.failure(StandardError::new(
                    "expected '{'",
                    self.current_span(),
                    Some("add a '{' to define the body"),
                ));
            }

            parse_result.register_advancement();
            self.advance();

            let body = parse_result.register(self.statements());

            if parse_result.error.is_some() {
                return parse_result;
            }

            if self.current_token_ref().token_type != TokenType::TT_RBRACKET {
                return parse_result.failure(StandardError::new(
                    "expected '}'",
                    self.current_span(),
                    Some("add a '}' to close the body"),
                ));
            }

            parse_result.register_advancement();
            self.advance();

            parse_result.success(self.arena.for_node(
                var_name,
                start_value,
                end_value,
                step_value,
                body,
            ))
        } else if self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "through")
        {
            parse_result.register_advancement();
            self.advance();

            let iterator = parse_result.register(self.expr());

            if parse_result.error.is_some() {
                return parse_result;
            }

            if self.current_token_ref().token_type != TokenType::TT_LBRACKET {
                return parse_result.failure(StandardError::new(
                    "expected '{'",
                    self.current_span(),
                    Some("add a '{' to define the body"),
                ));
            }

            parse_result.register_advancement();
            self.advance();

            let body = parse_result.register(self.statements());

            if parse_result.error.is_some() {
                return parse_result;
            }

            if self.current_token_ref().token_type != TokenType::TT_RBRACKET {
                return parse_result.failure(StandardError::new(
                    "expected '}'",
                    self.current_span(),
                    Some("add a '}' to close the body"),
                ));
            }

            parse_result.register_advancement();
            self.advance();

            parse_result.success(self.arena.for_each_node(var_name, iterator, body))
        } else {
            return parse_result.failure(StandardError::new(
                "expected '=' or 'through'",
                self.current_span(),
                None,
            ));
        }
    }

    fn while_expr(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();

        if !self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "while")
        {
            return parse_result.failure(StandardError::new(
                "expected keyword",
                self.current_span(),
                Some("add the 'while' keyword to initialize a while loop"),
            ));
        }

        parse_result.register_advancement();
        self.advance();

        let condition = parse_result.register(self.expr());

        if parse_result.error.is_some() {
            return parse_result;
        }

        if self.current_token_ref().token_type != TokenType::TT_LBRACKET {
            return parse_result.failure(StandardError::new(
                "expected '{'",
                self.current_span(),
                Some("add a '{' to define the body"),
            ));
        }

        parse_result.register_advancement();
        self.advance();

        let body = parse_result.register(self.statements());

        if parse_result.error.is_some() {
            return parse_result;
        }

        if self.current_token_ref().token_type != TokenType::TT_RBRACKET {
            return parse_result.failure(StandardError::new(
                "expected '}'",
                self.current_span(),
                Some("add a '}' to close the body"),
            ));
        }

        parse_result.register_advancement();
        self.advance();

        parse_result.success(self.arena.while_node(condition, body))
    }

    fn try_expr(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();

        if !self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "try")
        {
            return parse_result.failure(StandardError::new(
                "expected keyword",
                self.current_span(),
                Some("add the 'try' keyword to isolate unsafe code"),
            ));
        }

        parse_result.register_advancement();
        self.advance();

        if self.current_token_ref().token_type != TokenType::TT_LBRACKET {
            return parse_result.failure(StandardError::new(
                "expected '{'",
                self.current_span(),
                Some("add a '{' to define the body"),
            ));
        }

        parse_result.register_advancement();
        self.advance();

        let try_body = parse_result.register(self.statements());

        if parse_result.error.is_some() {
            return parse_result;
        }

        if self.current_token_ref().token_type != TokenType::TT_RBRACKET {
            return parse_result.failure(StandardError::new(
                "expected '}'",
                self.current_span(),
                Some("add a '}' to close the body"),
            ));
        }

        parse_result.register_advancement();
        self.advance();

        if !self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "catch")
        {
            return parse_result.failure(StandardError::new(
                "expected keyword",
                self.current_span(),
                Some("add the 'catch' keyword to isolate the safe code to execute if the 'try' block fails"),
            ));
        }

        parse_result.register_advancement();
        self.advance();

        if self.current_token_ref().token_type != TokenType::TT_IDENTIFIER {
            return parse_result.failure(StandardError::new(
                "expected identifier",
                self.current_span(),
                Some("add a name for caught error like 'error'"),
            ));
        }

        let error_name_token = self.current_token_copy();

        parse_result.register_advancement();
        self.advance();

        if self.current_token_ref().token_type != TokenType::TT_LBRACKET {
            return parse_result.failure(StandardError::new(
                "expected '{'",
                self.current_span(),
                Some("add a '{' to define the body"),
            ));
        }

        parse_result.register_advancement();
        self.advance();

        let except_body = parse_result.register(self.statements());

        if parse_result.error.is_some() {
            return parse_result;
        }

        if self.current_token_ref().token_type != TokenType::TT_RBRACKET {
            return parse_result.failure(StandardError::new(
                "expected '}'",
                self.current_span(),
                Some("add a '}' to close the body"),
            ));
        }

        parse_result.register_advancement();
        self.advance();

        parse_result.success(
            self.arena
                .try_except_node(try_body, except_body, error_name_token),
        )
    }

    fn import_expr(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();

        if !self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "fetch")
        {
            return parse_result.failure(StandardError::new(
                "expected keyword",
                self.current_span(),
                Some("add the 'fetch' keyword to import other '.glang' files"),
            ));
        }

        parse_result.register_advancement();
        self.advance();

        let import = parse_result.register(self.expr());

        if parse_result.error.is_some() {
            return parse_result;
        }

        parse_result.register_advancement();
        self.advance();

        parse_result.success(self.arena.import_node(import))
    }

    fn expr(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let next_tok = match self.next_token_copy() {
            Some(tok) => tok,
            None => Token::new(TokenType::TT_EOF, None, Span::empty()),
        };

        if self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "obj")
        {
            parse_result.register_advancement();
            self.advance();

            if self.current_token_copy().token_type != TokenType::TT_IDENTIFIER {
                return parse_result.failure(StandardError::new(
                    "expected identifier",
                    self.current_span(),
                    Some("add a name for this object like 'hotdog'"),
                ));
            }

            let var_name = self.current_token_copy();

            parse_result.register_advancement();
            self.advance();

            if self.current_token_copy().token_type != TokenType::TT_EQ {
                return parse_result.failure(StandardError::new(
                    "expected '='",
                    self.current_span(),
                    Some(
                        format!(
                            "add an '=' to set the value of the variable '{}'",
                            &var_name.value.unwrap()
                        )
                        .as_str(),
                    ),
                ));
            }

            parse_result.register_advancement();
            self.advance();

            let expr = parse_result.register(self.expr());

            if parse_result.error.is_some() {
                return parse_result;
            }

            return parse_result.success(self.arena.variable_assign_node(var_name, expr));
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

            return parse_result.success(self.arena.variable_reassign_node(var_name, expr));
        } else if self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "stay")
        {
            parse_result.register_advancement();
            self.advance();

            if self.current_token_copy().token_type != TokenType::TT_IDENTIFIER {
                return parse_result.failure(StandardError::new(
                    "expected identifier",
                    self.current_span(),
                    Some("add a name for this constant like 'HOT_DOG'"),
                ));
            }

            let const_name = self.current_token_copy();

            parse_result.register_advancement();
            self.advance();

            if self.current_token_copy().token_type != TokenType::TT_EQ {
                return parse_result.failure(StandardError::new(
                    "expected '='",
                    self.current_span(),
                    Some(
                        format!(
                            "add an '=' to set the value of the constant '{}'",
                            &const_name.value.unwrap()
                        )
                        .as_str(),
                    ),
                ));
            }

            parse_result.register_advancement();
            self.advance();

            let expr = parse_result.register(self.expr());

            if parse_result.error.is_some() {
                return parse_result;
            }

            return parse_result.success(self.arena.const_assign_node(const_name, expr));
        }

        let node = parse_result.register(self.binary_operator(
            Operator::ComparisonExpr,
            &[
                (TokenType::TT_KEYWORD, "and"),
                (TokenType::TT_KEYWORD, "or"),
            ],
            None,
        ));

        if parse_result.error.is_some() {
            return parse_result.failure(StandardError::new(
                "expected keyword, object, function, expression",
                self.current_span(),
                None,
            ));
        }

        parse_result.success(node)
    }

    fn statement(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let pos_start = self.current_position_start();

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

            return parse_result.success(self.arena.return_node(
                expr,
                Span::new(
                    &self.current_span().filename,
                    pos_start,
                    self.current_position_end(),
                ),
            ));
        } else if self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "next")
        {
            parse_result.register_advancement();
            self.advance();

            return parse_result.success(self.arena.continue_node(Span::new(
                &self.current_span().filename,
                pos_start,
                self.current_position_end(),
            )));
        } else if self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "leave")
        {
            parse_result.register_advancement();
            self.advance();

            return parse_result.success(self.arena.break_node(Span::new(
                &self.current_span().filename,
                pos_start,
                self.current_position_end(),
            )));
        }

        let expr = parse_result.register(self.expr());

        if parse_result.error.is_some() {
            return parse_result.failure(StandardError::new(
                "expected keyword, object, function, expression",
                Span::new(
                    &self.current_span().filename,
                    pos_start,
                    self.current_position_end(),
                ),
                None,
            ));
        }

        parse_result.success(expr)
    }

    fn statements(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let mut statements: Vec<NodeID> = Vec::new();
        let pos_start = self.current_position_start();

        if self.current_token_ref().token_type == TokenType::TT_EOF {
            return parse_result.success(self.arena.list_node(
                Vec::new(),
                Span::new(
                    &self.current_span().filename,
                    pos_start,
                    self.current_position_end(),
                ),
            ));
        }

        let statement = parse_result.register(self.statement());

        if parse_result.error.is_some() {
            return parse_result;
        }

        statements.push(statement);

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

            statements.push(statement);
        }

        parse_result.success(self.arena.list_node(
            statements,
            Span::new(
                &self.current_span().filename,
                pos_start,
                self.current_position_end(),
            ),
        ))
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

            let mut arg_nodes: Vec<NodeID> = Vec::new();
            let closing_call: Token;

            if self.current_token_ref().token_type == TokenType::TT_RPAREN {
                closing_call = self.current_token_copy();

                parse_result.register_advancement();
                self.advance();
            } else {
                let expr = parse_result.register(self.expr());

                if parse_result.error.is_some() {
                    return parse_result.failure(StandardError::new(
                        "expected keyword, object, function, expression",
                        self.current_span(),
                        None,
                    ));
                }

                arg_nodes.push(expr);

                while self.current_token_ref().token_type == TokenType::TT_COMMA {
                    parse_result.register_advancement();
                    self.advance();

                    let expr = parse_result.register(self.expr());

                    if parse_result.error.is_some() {
                        return parse_result;
                    }

                    arg_nodes.push(expr);
                }

                if self.current_token_ref().token_type != TokenType::TT_RPAREN {
                    return parse_result.failure(StandardError::new(
                        "expected ',' or ')'",
                        self.current_span(),
                        Some("add a ',' to input all the function arguments or close with a ')' to call the function"),
                    ));
                }

                closing_call = self.current_token_copy();

                parse_result.register_advancement();
                self.advance();
            }

            return parse_result.success(self.arena.call_node(atom, arg_nodes, closing_call));
        }

        parse_result.success(atom)
    }

    fn atom(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let token = self.current_token_copy();

        if token.token_type == TokenType::TT_NUM {
            parse_result.register_advancement();
            self.advance();

            return parse_result.success(self.arena.number_node(token));
        } else if token.token_type == TokenType::TT_STR {
            parse_result.register_advancement();
            self.advance();

            return parse_result.success(self.arena.string_node(token));
        } else if token.token_type == TokenType::TT_IDENTIFIER {
            parse_result.register_advancement();
            self.advance();

            return parse_result.success(self.arena.variable_access_node(token));
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
                return parse_result.failure(StandardError::new(
                    "expected closing parenthesis",
                    self.current_span(),
                    Some("add a ')' to close the original '('"),
                ));
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
        } else if token.matches(TokenType::TT_KEYWORD, "try") {
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

        parse_result.failure(StandardError::new(
            "expected object, keyword, function, or expression",
            token.span,
            None,
        ))
    }

    fn power(&mut self) -> ParseResult {
        self.binary_operator(
            Operator::Call,
            &[(TokenType::TT_POW, "")],
            Some(Operator::Factor),
        )
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

            return parse_result.success(self.arena.unary_operator_node(token, factor));
        }

        self.power()
    }

    fn term(&mut self) -> ParseResult {
        self.binary_operator(
            Operator::Factor,
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
            return parse_result.failure(StandardError::new(
                "expected keyword",
                self.current_span(),
                Some("add the 'func' keyword to define a function"),
            ));
        }

        parse_result.register_advancement();
        self.advance();

        let var_name_token: Option<Token>;

        if self.current_token_ref().token_type == TokenType::TT_IDENTIFIER {
            var_name_token = Some(self.current_token_copy());
            parse_result.register_advancement();
            self.advance();

            if self.current_token_ref().token_type != TokenType::TT_LPAREN {
                return parse_result.failure(StandardError::new(
                    "expected '('",
                    self.current_span(),
                    Some("add a '(' to define the function arguments"),
                ));
            }
        } else {
            var_name_token = None;

            if self.current_token_ref().token_type != TokenType::TT_LPAREN {
                return parse_result.failure(StandardError::new(
                    "expected identifier or '('",
                    self.current_span(),
                    Some("add a name for this function like 'greet' or use '(' to define an anonymous function"),
                ));
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
                    return parse_result.failure(StandardError::new(
                        "expected identifier",
                        self.current_span(),
                        Some("add a name for the function arguments like 'name'"),
                    ));
                }

                arg_name_tokens.push(self.current_token_copy());

                parse_result.register_advancement();
                self.advance();
            }

            if self.current_token_ref().token_type != TokenType::TT_RPAREN {
                return parse_result.failure(StandardError::new(
                    "expected ')'",
                    self.current_span(),
                    Some("add another function argument or complete the function with ')'"),
                ));
            }
        } else if self.current_token_ref().token_type != TokenType::TT_RPAREN {
            return parse_result.failure(StandardError::new(
                "expected indentifier or ')'",
                self.current_span(),
                Some("add a name for the function arguments like 'name' or complete the function with ')'"),
            ));
        }

        parse_result.register_advancement();
        self.advance();

        if self.current_token_ref().token_type != TokenType::TT_LBRACKET {
            return parse_result.failure(StandardError::new(
                "expected '{'",
                self.current_span(),
                Some("add a '{' to define the body of the function"),
            ));
        }

        parse_result.register_advancement();
        self.advance();

        let body = parse_result.register(self.statements());

        if parse_result.error.is_some() {
            return parse_result;
        }

        if self.current_token_ref().token_type != TokenType::TT_RBRACKET {
            return parse_result.failure(StandardError::new(
                "expected '}'",
                self.current_span(),
                Some("add a '}' to close the body"),
            ));
        }

        parse_result.register_advancement();
        self.advance();

        parse_result.success(self.arena.function_definition_node(
            var_name_token,
            &arg_name_tokens,
            body,
            false,
        ))
    }

    fn binary_operator(
        &mut self,
        func_a: Operator,
        ops: &[(TokenType, &str)],
        func_b: Option<Operator>,
    ) -> ParseResult {
        let func_b = func_b.unwrap_or(func_a.clone());

        let mut parse_result = ParseResult::new();
        let mut left = parse_result.register(match func_a {
            Operator::ComparisonExpr => self.comparison_expr(),
            Operator::ArithmeticExpr => self.arithmetic_expr(),
            Operator::Term => self.term(),
            Operator::Factor => self.factor(),
            Operator::Call => self.call(),
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
                Operator::ComparisonExpr => self.comparison_expr(),
                Operator::ArithmeticExpr => self.arithmetic_expr(),
                Operator::Term => self.term(),
                Operator::Factor => self.factor(),
                Operator::Call => self.call(),
            });

            if parse_result.error.is_some() {
                return parse_result;
            }

            left = self.arena.binary_operator_node(left, op_token, right);
        }

        parse_result.success(left)
    }
}

// Test the output AST from parsed tokens
#[test]
fn test_ast() {
    use crate::AstNode;
    use glang_lexer::Lexer;
    use std::path::Path;

    let code = "function(1 + 1);";

    let mut lexer = Lexer::new(Path::new("<test>"), code);
    let tokens = lexer.make_tokens().ok().unwrap();

    let mut parser = Parser::new(&tokens, lexer.contents());
    let ast = parser.parse();

    let node = match parser.arena.get(ast.node) {
        AstNode::List(l) => l,
        _ => panic!("Expected a list node"),
    };

    assert_eq!(node.element_nodes.len(), 1); // only one call node
    assert!(matches!(
        parser.arena.get(node.element_nodes[0]),
        AstNode::Call { .. }
    ));
}
