use std::ops::Range;

use super::expr::Expr;

#[derive(Debug)]
pub(crate) struct Stmt {
    pub kind: StmtKind,
    pub span: Range<usize>,
}

#[derive(Debug)]
pub(crate) enum StmtKind {
    ConstAssignment { symbol: Expr, value: Expr },
    Assignment { symbol: Expr, value: Expr },
    Reassignment { symbol: Expr, value: Expr },
}
