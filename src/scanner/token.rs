use std::{fmt::Display, ops::Range};

#[derive(Debug, PartialEq)]
pub(crate) struct Token {
    pub kind: TokenKind,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum TokenKind {
    // Grouping symbols
    LPar,
    RPar,
    LBrac,
    RBrac,
    LCurl,
    RCurl,

    // Mathematical operators
    Plus,
    Minus,
    Star,
    Slash,
    Modulo,
    Caret,
    PlusPlus,
    PlusEqual,
    MinusMinus,
    MinusEqual,
    StarEqual,

    // Comparison operators
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    More,
    MoreEqual,
    Amp, // ampsersand &
    AmpAmp,
    Bar, // pipe |
    BarBar,

    // Other symbols
    LArrow,
    RArrow,
    SlashSlash,
    At,
    Hash,
    Dollar,
    Semicolon,
    Question,
    Comma,
    Dot,
    Colon,

    // Literals
    Ident(String),
    Number(String),
    String(String),

    // Keywords
    Const,
    Function,
    End,
    While,
    For,
    If,
    Then,
    Else,
    Elif,
    Do,
    Break,
    Continue,
    EOF,
}
