use std::ops::Range;

use crate::token::TokenKind;

use super::parser::Parser;

#[derive(Debug)]
pub(crate) struct Expr {
    pub kind: ExprKind,
    pub span: Range<usize>,
}

#[derive(Debug)]
pub(crate) enum ExprKind {
    Operator {
        op: Operator,
    },
    Number {
        v: f64,
    },
    String {
        v: String,
    },
    Symbol {
        ident: String,
    },
    TypedSymbol {
        symbol: Box<Expr>,
        stype: Box<Expr>,
    },
    IndexInto {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    FunctionCall {
        symbol: Box<Expr>,
        args: Vec<Box<Expr>>,
    },
    BinaryExpr {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        op: Operator,
    },
}

impl Parser {
    pub(in crate::ast) fn parse_expr(&mut self) -> Option<Expr> {
        if let Some(tk) = self.next() {
            return match tk.kind {
                TokenKind::Ident(s) => Some(self.parse_symbol(s)),
                _ => None,
            };
        }

        None
    }

    fn parse_symbol(&mut self, ident: String) -> Expr {
        let lhs = self.create_expr(ExprKind::Symbol { ident });

        // We already have the symbol name, s, and it's token has been
        // consumed. We want to check to make sure this symbol isn't being indexed into
        if let Some(tk) = self.peek() {
            match tk.kind {
                TokenKind::Dot => {
                    // Consume the dot
                    let _ = self.next();

                    // Parse another expr and expect it to be a
                    let rhs = self.parse_expr().unwrap_or_else(|| {
                        panic!("Dot found, expected a valid symbol to index into this symobl");
                    });

                    // Assert that RHS is a symbol or similar
                    match &rhs.kind {
                        ExprKind::IndexInto { lhs: _, rhs: _ } | ExprKind::Symbol { ident: _ } => {}
                        _ => panic!("Expected a symbol or indexing into a symbol after dot!"),
                    }

                    return self.create_expr(ExprKind::IndexInto {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    });
                }
                TokenKind::Colon => {
                    // Consume the colon
                    let _ = self.next();

                    // Parse another expr and expect it to be a
                    let stype = self.parse_expr().unwrap_or_else(|| {
                        panic!("Dot found, expected a valid symbol to annotate this symobl");
                    });

                    // Assert that stype is a symbol or similar
                    match &stype.kind {
                        ExprKind::IndexInto { lhs: _, rhs: _ } | ExprKind::Symbol { ident: _ } => {}
                        _ => panic!("Expected a symbol or indexing into a symbol after colon!"),
                    }

                    return self.create_expr(ExprKind::TypedSymbol {
                        symbol: Box::new(lhs),
                        stype: Box::new(stype),
                    });
                }
                _ => {}
            }
        }

        return lhs;
    }

    fn create_expr(&mut self, kind: ExprKind) -> Expr {
        Expr {
            kind,
            span: self.span_now(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Operator {
    pub op_kind: OperatorKind,
    pub assoc: Associativity,
    pub precedence: u8,
}

impl Operator {
    pub(crate) fn from_token(tk: &TokenKind) -> Option<Self> {
        match tk {
            TokenKind::ParOpen => Some(Operator {
                op_kind: OperatorKind::ParOpen,
                assoc: Associativity::Right,
                precedence: 255,
            }),
            TokenKind::ParClose => Some(Operator {
                op_kind: OperatorKind::ParClose,
                assoc: Associativity::Left,
                precedence: 255,
            }),
            TokenKind::Plus => Some(Operator {
                op_kind: OperatorKind::Plus,
                assoc: Associativity::Left,
                precedence: 2,
            }),
            TokenKind::Minus => Some(Operator {
                op_kind: OperatorKind::Minus,
                assoc: Associativity::Left,
                precedence: 2,
            }),
            TokenKind::Star => Some(Operator {
                op_kind: OperatorKind::Multiply,
                assoc: Associativity::Left,
                precedence: 3,
            }),
            TokenKind::Slash => Some(Operator {
                op_kind: OperatorKind::Divide,
                assoc: Associativity::Left,
                precedence: 3,
            }),
            TokenKind::Equal => Some(Operator {
                op_kind: OperatorKind::Assignment,
                assoc: Associativity::Right,
                precedence: 0,
            }),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum OperatorKind {
    Plus,
    Minus,
    Divide,
    Multiply,
    Exponent,
    Modulo,
    ParOpen,
    ParClose,
    Assignment,
}

#[derive(Debug)]
pub(crate) enum Associativity {
    Left,
    Right,
}
