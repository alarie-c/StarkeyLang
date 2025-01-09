use std::ops::Range;

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
    BinaryExpr { lhs: Box<Expr>, rhs: Box<Expr>, op: Operator}
}

pub(crate) struct Operator {
    op_kind: OperatorKind,
    assoc: Associativity,
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
