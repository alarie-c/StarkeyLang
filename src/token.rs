use std::{fmt::Display, ops::Range};

#[derive(Debug, PartialEq)]
pub(crate) struct Token {
    pub kind: TokenKind,
    pub span: Range<usize>,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let span = format!("[{}..{}]", self.span.start, self.span.end);
        match &self.kind {
            TokenKind::ParOpen => write!(f, "{}: OPEN PAR '('", span),
            TokenKind::ParClose => write!(f, "{}: CLOSE PAR ')'", span),
            TokenKind::BracOpen => write!(f, "{}: OPEN BRAC '['", span),
            TokenKind::BracClose => write!(f, "{}: CLOSE BRAC ']'", span),
            TokenKind::CurlOpen => write!(f, "{}: OPEN CURL '{{'", span),
            TokenKind::CurlClose => write!(f, "{}: CLOSE CURL '}}'", span),
            TokenKind::Plus => write!(f, "{}: PLUS '+'", span),
            TokenKind::Minus => write!(f, "{}: MINUS '-'", span),
            TokenKind::Star => write!(f, "{}: STAR '*'", span),
            TokenKind::Slash => write!(f, "{}: SLASH '/'", span),
            TokenKind::Modulo => write!(f, "{}: MODULO '%'", span),
            TokenKind::Caret => write!(f, "{}: CARET '^'", span),
            TokenKind::DoublePlus => write!(f, "{}: DOUBLE PLUS '++'", span),
            TokenKind::PlusEqual => write!(f, "{}: PLUS EQUAL '+='", span),
            TokenKind::DoubleMinus => write!(f, "{}: DOUBLE MINUS '--'", span),
            TokenKind::MinusEqual => write!(f, "{}: MINUS EQUAL '-='", span),
            TokenKind::StarEqual => write!(f, "{}: STAR EQUAL '*='", span),
            TokenKind::Equal => write!(f, "{}: EQUAL '='", span),
            TokenKind::DoubleEqual => write!(f, "{}: DOUBLE EQUAL '=='", span),
            TokenKind::Bang => write!(f, "{}: BANG '!'", span),
            TokenKind::BangEqual => write!(f, "{}: BANG EQUAL '!='", span),
            TokenKind::LessThan => write!(f, "{}: LESS THAN '<'", span),
            TokenKind::LessThanEqual => write!(f, "{}: LESS THAN EQUAL '<='", span),
            TokenKind::MoreThan => write!(f, "{}: MORE THAN '>'", span),
            TokenKind::MoreThanEqual => write!(f, "{}: MORE THAN EQUAL '>='", span),
            TokenKind::Amp => write!(f, "{}: AMPERSAND '&'", span),
            TokenKind::DoubleAmp => write!(f, "{}: DOUBLE AMPERSAND '&&'", span),
            TokenKind::Bar => write!(f, "{}: BAR '|'", span),
            TokenKind::DoubleBar => write!(f, "{}: DOUBLE BAR '||'", span),
            TokenKind::Arrow => write!(f, "{}: ARROW '->'", span),
            TokenKind::DoubleSlash => write!(f, "{}: DOUBLE SLASH '//'", span),
            TokenKind::At => write!(f, "{}: AT '@'", span),
            TokenKind::Hash => write!(f, "{}: HASH '#'", span),
            TokenKind::Dollar => write!(f, "{}: DOLLAR '%'", span),
            TokenKind::Semicolon => write!(f, "{}: SEMICOLON ';'", span),
            TokenKind::Question => write!(f, "{}: QUESTION '?'", span),
            TokenKind::Comma => write!(f, "{}: COMMA ','", span),
            TokenKind::Ident(s) => write!(f, "{}: IDENTIFIER '{s}'", span),
            TokenKind::Number(s) => write!(f, "{}: NUMBER '{s}'", span),
            TokenKind::String(s) => write!(f, "{}: STRING '{s}'", span),
            TokenKind::Const => write!(f, "{}: CONST", span),
            TokenKind::Function => write!(f, "{}: FUNCTION", span),
            TokenKind::End => write!(f, "{}: END", span),
            TokenKind::While => write!(f, "{}: WHILE", span),
            TokenKind::For => write!(f, "{}: FOR", span),
            TokenKind::If => write!(f, "{}: IF", span),
            TokenKind::Then => write!(f, "{}: THEN", span),
            TokenKind::Else => write!(f, "{}: ELSE", span),
            TokenKind::Elif => write!(f, "{}: ELIF", span),
            TokenKind::Do => write!(f, "{}: DO", span),
            TokenKind::Break => write!(f, "{}: BREAK", span),
            TokenKind::Continue => write!(f, "{}: CONTINUE", span),
            TokenKind::EOF => write!(f, "{}: EOF", span),
        }
    }
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
