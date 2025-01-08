use std::ops::Range;

#[derive(Debug, PartialEq)]
pub(crate) struct Token {
    pub kind: TokenKind,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum TokenKind {
    ParOpen,
    ParClose,
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    EqualEqual,

    Ident(String),
    Number(String),
    String(String),

    Const,
    EOF,
}
