use std::ops::Range;

use crate::scanner::token::Token;

pub(crate) struct Node {
    inner: Expr,
    span: Range<usize>,
}

pub(crate) enum Expr {
    // Values and terminals
    NullLiteral,
    StringLiteral { value: String },
    NumberLiteral { value: f64 },
    BooleanLiteral { value: bool },
    Symbol { ident: String },
    
    // Other crap
    GroupedExpr { lpar: Token, expr: Box<Expr>, rpar: Token },
    UnaryExpr { op: Token, rhs: Box<Expr> },
    BinaryExpr { lhs: Box<Expr>, op: Token, rhs: Box<Expr> },
    AssignExpr { symbol: Box<Expr>, rhs: Box<Expr> },
    MutAssignExpr { symbol: Box<Expr>, rhs: Box<Expr> },
}