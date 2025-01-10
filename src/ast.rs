use std::ops::Range;

use crate::token::TokenKind;

pub(crate) struct AbstractSyntaxTree(Vec<Node>);

impl AbstractSyntaxTree {
    pub(crate) fn new() -> Self {
        Self(vec![])
    }

    pub(crate) fn push(&mut self, node: Node) {
        self.0.push(node);
    }
}

pub(crate) struct Node {
    pub expr: Expr,
    pub span: Range<usize>,
}

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
    AccessMember {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
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

pub(crate) struct Operator {
    op_kind: OperatorKind,
    assoc: Associativity,
    precedence: u8,
}

impl Operator {
    pub(crate) fn fromtk(tk: &TokenKind) -> Option<Self> {
        match tk {
            TokenKind::Plus => Some(Operator {
                op_kind: OperatorKind::Plus,
                assoc: Associativity::Left,
                precedence: 3,
            }),
            TokenKind::Minus => Some(Operator {
                op_kind: OperatorKind::Minus,
                assoc: Associativity::Left,
                precedence: 3,
            }),
            TokenKind::Star => Some(Operator {
                op_kind: OperatorKind::Multiply,
                assoc: Associativity::Left,
                precedence: 2,
            }),
            TokenKind::Slash => Some(Operator {
                op_kind: OperatorKind::Divide,
                assoc: Associativity::Left,
                precedence: 2,
            }),
            _ => None,
        }
    }
}

pub(crate) enum OperatorKind {
    Plus,
    Minus,
    Divide,
    Multiply,
    Exponent,
    Modulo,
}

pub(crate) enum Associativity {
    Left,
    Right,
}
