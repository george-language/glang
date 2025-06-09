use crate::errors::standard_error::StandardError;

#[derive(Clone)]
pub struct ParseResult {
    pub error: Option<StandardError>,
    pub node: Option<String>,
    pub last_registered_advance_count: usize,
    pub advance_count: usize,
    pub to_reverse_count: usize,
}

impl ParseResult {
    pub fn new() -> Self {
        ParseResult {
            error: None,
            node: None,
            last_registered_advance_count: 0,
            advance_count: 0,
            to_reverse_count: 0,
        }
    }

    pub fn register_advancement(&mut self) {
        self.last_registered_advance_count = 1;
        self.advance_count += 1;
    }

    pub fn register(&mut self, parse_result: ParseResult) -> Option<String> {
        self.last_registered_advance_count = parse_result.advance_count;
        self.advance_count += parse_result.advance_count;

        if let Some(_) = parse_result.error {
            self.error = parse_result.error
        }

        parse_result.node
    }

    pub fn try_register(&mut self, parse_result: ParseResult) {
        if let Some(_) = parse_result.error {
            self.to_reverse_count = parse_result.advance_count
        }
    }
}
