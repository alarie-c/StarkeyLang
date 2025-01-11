use std::ops::Range;

use crate::token::TokenKind;

#[derive(Debug)]
pub(crate) struct AbstractSyntaxTree(Vec<Node>);

impl AbstractSyntaxTree {
    pub(crate) fn new() -> Self {
        Self(vec![])
    }

    pub(crate) fn push_node(&mut self, node: Node) {
        self.0.push(node);
    }

    pub(crate) fn pull_node(&mut self) -> Option<Node> {
        self.0.pop()
    }
}

#[derive(Debug)]
pub(crate) struct Node {
    pub expr: Expr,
    pub span: Range<usize>,
}

#[derive(Debug)]
pub(crate) enum Expr {
    Symbol {
        name: String,
    },
    Number {
        value: f64,
    },
    Operator {
        op: Operator,
    },
    IndexInto {
        item: Box<Expr>,
        index: Box<Expr>,
    },
    BinaryExpr {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        op: Operator,
    },
    EndOfFile {
        code: i32,
    },
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
}

#[derive(Debug)]
pub(crate) enum Associativity {
    Left,
    Right,
}
