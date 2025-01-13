use std::ops::Range;

use crate::token::TokenKind;

use super::expr::{Expr, ExprKind, Operator, OperatorKind};
use super::parser::Parser;

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
    FunctionalCall { call: Expr },
}

impl Parser {
    pub(in crate::ast) fn parse_stmt(&mut self) -> Option<Stmt> {
        if let Some(tk) = self.next() {
            return match tk.kind {
                TokenKind::If | TokenKind::Else | TokenKind::Elif => todo!("Selection"),
                TokenKind::While => todo!("Control flow"),
                _ => None,
            };
        }

        None
    }

    fn read_stmt(&mut self) -> Option<Stmt> {
        let mut exprs = Vec::<Expr>::new();
        while let None = self.peek_assert(TokenKind::Semicolon) {
            match self.parse_expr() {
                Some(e) => exprs.push(e),
                None => continue,
            }
        }

        // Traverse these expressions
        let tree = self.build_tree(exprs);

        None
    }

    fn build_tree(&mut self, mut exprs: Vec<Expr>) -> Vec<Expr> {
        let mut tree: Vec<Expr> = vec![];

        // Iterate through every expr and construct a tree
        exprs.drain(0..).for_each(|e| match e.kind {
            ExprKind::Operator { op } => match self.from_operator(op, &mut tree) {
                Some(new_expr) => tree.push(new_expr),
                None => {}
            },
            _ => tree.push(e),
        });

        return tree;
    }

    fn from_operator(&mut self, op: Operator, tree: &mut Vec<Expr>) -> Option<Expr> {
        match &op.op_kind {
            OperatorKind::Plus
            | OperatorKind::Minus
            | OperatorKind::Multiply
            | OperatorKind::Divide => {
                let rhs = tree.pop().unwrap_or_else(|| {
                    panic!("Expected valid RHS expr for binary expr");
                });
                let lhs = tree.pop().unwrap_or_else(|| {
                    panic!("Expected valid LHS expr for binary expr");
                });

                // Return the binary expressions
                return Some(Expr {
                    kind: ExprKind::BinaryExpr {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                        op,
                    },
                    span: self.span_now(),
                });
            }
            _ => todo!("Unknown operator, {:?}", op),
        }
    }
}
