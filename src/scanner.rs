use std::{collections::VecDeque, ops::Range};

use crate::token::{Token, TokenKind};

const WHITESPACE: &'static str = " \r\n\t";

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

    pub(crate) fn scan_next(&mut self) -> Option<Token> {
        if self.next() {
            while WHITESPACE.contains(self.current) {
                if !self.next() {
                    return Some(self.token(TokenKind::EOF));
                }
            }

            return match self.current {
                '(' => Some(self.token(TokenKind::ParOpen)),
                ')' => Some(self.token(TokenKind::ParClose)),
                '=' => {
                    if self.assert_next('=') {
                        Some(self.token(TokenKind::EqualEqual))
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
