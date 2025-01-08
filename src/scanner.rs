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

            // Match this character
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

    fn peek(&self) -> Option<char> {
        self.stream.front().copied()
    }

    unsafe fn consume(&mut self) {
        self.current = unsafe { self.stream.pop_front().unwrap_unchecked() };
    }

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
