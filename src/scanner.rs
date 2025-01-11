use std::{collections::VecDeque, ops::Range};

use crate::token::{Token, TokenKind};

const WHITESPACE: &'static str = " \r\n\t";

#[derive(Debug)]
pub(crate) struct Scanner {
    stream: VecDeque<char>,
    current: char,
    span: Range<usize>,
}

impl Scanner {
    pub(crate) fn new(source: &String) -> Self {
        Self {
            stream: source.chars().collect(),
            current: ' ',
            span: 0..0,
        }
    }

    pub(crate) fn scan(&mut self) -> Vec<Token> {
        let mut output: Vec<Token> = vec![];
        while !self.stream.is_empty() {
            match self.scan_next() {
                Some(token) => output.push(token),
                None => panic!("Something went wrong while tokenizing"),
            }
        }
        if output.last().is_some_and(|t| t.kind != TokenKind::EOF) {
            output.push(self.token(TokenKind::EOF));
        }
        return output;
    }

    fn scan_next(&mut self) -> Option<Token> {
        if self.next() {
            while WHITESPACE.contains(self.current) {
                if !self.next() {
                    return Some(self.token(TokenKind::EOF));
                }
            }

            return match self.current {
                '(' => Some(self.token(TokenKind::ParOpen)),
                ')' => Some(self.token(TokenKind::ParClose)),
                '{' => Some(self.token(TokenKind::CurlOpen)),
                '}' => Some(self.token(TokenKind::CurlClose)),
                '[' => Some(self.token(TokenKind::BracOpen)),
                ']' => Some(self.token(TokenKind::BracClose)),
                ';' => Some(self.token(TokenKind::Semicolon)),
                '^' => Some(self.token(TokenKind::Caret)),
                '%' => Some(self.token(TokenKind::Modulo)),
                ',' => Some(self.token(TokenKind::Comma)),
                '$' => Some(self.token(TokenKind::Dollar)),
                '#' => Some(self.token(TokenKind::Hash)),
                '@' => Some(self.token(TokenKind::At)),
                '?' => Some(self.token(TokenKind::Question)),
                '.' => Some(self.token(TokenKind::Dot)),
                '&' => {
                    if self.assert_next('&') {
                        Some(self.token(TokenKind::DoubleAmp))
                    } else {
                        Some(self.token(TokenKind::Amp))
                    }
                }
                '|' => {
                    if self.assert_next('|') {
                        Some(self.token(TokenKind::DoubleBar))
                    } else {
                        Some(self.token(TokenKind::Bar))
                    }
                }
                '+' => {
                    if self.assert_next('+') {
                        Some(self.token(TokenKind::DoublePlus))
                    } else if self.assert_next('=') {
                        Some(self.token(TokenKind::PlusEqual))
                    } else {
                        Some(self.token(TokenKind::Plus))
                    }
                }
                '-' => {
                    if self.assert_next('>') {
                        Some(self.token(TokenKind::Arrow))
                    } else if self.assert_next('-') {
                        Some(self.token(TokenKind::DoubleMinus))
                    } else if self.assert_next('=') {
                        Some(self.token(TokenKind::MinusEqual))
                    } else {
                        Some(self.token(TokenKind::Minus))
                    }
                }
                '*' => {
                    if self.assert_next('=') {
                        Some(self.token(TokenKind::StarEqual))
                    } else {
                        Some(self.token(TokenKind::Star))
                    }
                }
                '/' => {
                    if self.assert_next('/') {
                        Some(self.token(TokenKind::DoubleSlash))
                    } else {
                        Some(self.token(TokenKind::Slash))
                    }
                }
                '<' => {
                    if self.assert_next('=') {
                        Some(self.token(TokenKind::LessThanEqual))
                    } else {
                        Some(self.token(TokenKind::LessThan))
                    }
                }
                '>' => {
                    if self.assert_next('=') {
                        Some(self.token(TokenKind::MoreThanEqual))
                    } else {
                        Some(self.token(TokenKind::MoreThan))
                    }
                }
                '!' => {
                    if self.assert_next('=') {
                        Some(self.token(TokenKind::BangEqual))
                    } else {
                        Some(self.token(TokenKind::Bang))
                    }
                }
                '=' => {
                    if self.assert_next('=') {
                        Some(self.token(TokenKind::DoubleEqual))
                    } else {
                        Some(self.token(TokenKind::Equal))
                    }
                }
                '"' => {
                    let mut buffer = String::new();
                    self.next();
                    while self.current != '"' {
                        // Check for EOF!
                        if self.stream.len() == 0 {
                            eprintln!("Non-terminating string literal!");
                            break;
                        }

                        buffer.push(self.current);
                        self.next();
                    }
                    return Some(self.token(TokenKind::String(buffer)));
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut buffer = String::from(self.current);
                    while self.assert_ident() {
                        buffer.push(self.current);
                    }
                    return Some(self.match_keywords(buffer));
                }
                '0'..='9' => {
                    let mut buffer = String::from(self.current);
                    while self.assert_number() {
                        buffer.push(self.current);
                    }
                    return Some(self.token(TokenKind::Number(buffer)));
                }
                _ => None,
            };
        }
        return Some(self.token(TokenKind::EOF));
    }

    /// Attempt to match keywords from what would otherwise be an identifier
    /// If the identifier doesn't match any keywords, this functions will just
    /// return an Identifier token
    fn match_keywords(&mut self, stream: String) -> Token {
        match stream.as_str() {
            "const" => self.token(TokenKind::Const),
            "if" => self.token(TokenKind::If),
            "then" => self.token(TokenKind::Then),
            "else" => self.token(TokenKind::Else),
            "elif" => self.token(TokenKind::Elif),
            "while" => self.token(TokenKind::While),
            "do" => self.token(TokenKind::Do),
            "for" => self.token(TokenKind::For),
            "function" => self.token(TokenKind::Function),
            "end" => self.token(TokenKind::End),
            "continue" => self.token(TokenKind::Continue),
            "break" => self.token(TokenKind::Break),
            _ => self.token(TokenKind::Ident(stream)),
        }
    }

    fn assert_number(&mut self) -> bool {
        if let Some(peek) = self.peek() {
            if ('0'..='9').contains(&peek) || '_' == peek || '.' == peek {
                // Consume the char here
                unsafe { self.consume() };
                return true;
            }
        }
        return false;
    }

    fn assert_ident(&mut self) -> bool {
        if let Some(peek) = self.peek() {
            if ('a'..='z').contains(&peek)
                || ('A'..='Z').contains(&peek)
                || ('0'..='9').contains(&peek)
                || '_' == peek
            {
                // Consume the char here
                unsafe { self.consume() };
                return true;
            }
        }
        return false;
    }

    fn assert_next(&mut self, ch: char) -> bool {
        if let Some(peek) = self.peek() {
            if peek == ch {
                // Consume the char here
                unsafe { self.consume() };
                return true;
            }
        }
        return false;
    }

    /// Returns the front of the stream without consuming it
    fn peek(&self) -> Option<char> {
        self.stream.front().copied()
    }

    /// Pops the front of the stream and sets `self.current`
    /// to that character
    ///
    /// Must use this with some kind of bounds checking otherwise it will
    /// attempt to pop and unwrap even if there isn't anything left in the stream
    unsafe fn consume(&mut self) {
        self.current = unsafe { self.stream.pop_front().unwrap_unchecked() };
    }

    /// Tries to pop the front of the stream and will set `self.current`
    /// to that character if successful
    ///
    /// Returns true if pop was successful, returns false is it wasn't (signifies EOF)
    fn next(&mut self) -> bool {
        if let Some(ch) = self.stream.pop_front() {
            self.current = ch;
            true
        } else {
            false
        }
    }

    fn token(&mut self, kind: TokenKind) -> Token {
        Token {
            kind,
            span: self.span.clone(),
        }
    }
}
