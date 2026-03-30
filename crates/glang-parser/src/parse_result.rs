use crate::{AstNode, ListNode};
use glang_attributes::{Span, StandardError};

#[derive(Clone)]
pub struct ParseResult {
    pub error: Option<StandardError>,
    pub node: Box<AstNode>,
    pub last_registered_advance_count: usize,
    pub advance_count: usize,
    pub to_reverse_count: usize,
}

impl ParseResult {
    pub fn new() -> Self {
        Self {
            error: None,
            node: Box::new(AstNode::List(ListNode::new(&[], Span::empty()))),
            last_registered_advance_count: 0,
            advance_count: 0,
            to_reverse_count: 0,
        }
    }

    pub fn register_advancement(&mut self) {
        self.last_registered_advance_count = 1;
        self.advance_count += 1;
    }

    pub fn register(&mut self, parse_result: ParseResult) -> Box<AstNode> {
        self.last_registered_advance_count = parse_result.advance_count;
        self.advance_count += parse_result.advance_count;

        if parse_result.error.is_some() {
            self.error = parse_result.error
        }

        parse_result.node
    }

    pub fn try_register(&mut self, parse_result: ParseResult) -> Option<Box<AstNode>> {
        if parse_result.error.is_some() {
            self.to_reverse_count = parse_result.advance_count;

            return None;
        }

        Some(self.register(parse_result))
    }

    pub fn success(&mut self, node: Box<AstNode>) -> ParseResult {
        self.node = node;

        self.clone()
    }

    pub fn failure(&mut self, error: StandardError) -> ParseResult {
        if self.error.is_none() || self.last_registered_advance_count == 0 {
            self.error = Some(error)
        }

        self.clone()
    }
}
