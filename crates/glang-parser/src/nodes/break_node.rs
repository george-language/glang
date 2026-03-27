use glang_attributes::Span;

#[derive(Debug, Clone)]
pub struct BreakNode {
    pub span: Span,
}

impl BreakNode {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}
