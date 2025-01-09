use std::ops::Range;

#[derive(Debug, PartialEq)]
pub(crate) struct Token {
    pub kind: TokenKind,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum TokenKind {
    // Grouping symbols
    ParOpen,
    ParClose,
    BracOpen,
    BracClose,
    CurlOpen,
    CurlClose,

    // Mathematical operators
    Plus,
    Minus,
    Star,
    Slash,
    Modulo,
    Caret,
    DoublePlus,
    PlusEqual,
    DoubleMinus,
    MinusEqual,
    StarEqual,
    
    // Comparison operators
    Equal,
    DoubleEqual,
    Bang,
    BangEqual,
    LessThan,
    LessThanEqual,
    MoreThan,
    MoreThanEqual,
    Amp, // ampsersand &
    DoubleAmp,
    Bar, // pipe |
    DoubleBar,

    // Other symbols
    Arrow,
    DoubleSlash,
    At,
    Hash,
    Dollar,
    Semicolon,
    Question,
    Comma,

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
