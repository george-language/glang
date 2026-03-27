use glang_attributes::Span;

#[derive(Debug, Clone)]
pub struct ContinueNode {
    pub span: Span,
}

impl ContinueNode {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}
