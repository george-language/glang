#[derive(Debug, Clone)]
pub enum Operator {
    ComparisonExpr,
    ArithmeticExpr,
    Term,
    Factor,
    Call,
}
