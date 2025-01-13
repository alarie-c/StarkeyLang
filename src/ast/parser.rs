use std::ops::Range;

use crate::token::{Token, TokenKind};

use super::decl::Decl;

pub(crate) struct Parser {
    stream: Vec<Token>,
    decls: Vec<Decl>,
    span: Range<usize>,
}

impl Parser {
    pub(crate) fn new(stream: Vec<Token>) -> Self {
        Self {
            stream,
            decls: vec![],
            span: 0..0,
        }
    }

    pub(in crate::ast) fn next(&mut self) -> Option<Token> {
        self.stream.pop()
    }

    pub(in crate::ast) fn next_as(&mut self, kind: TokenKind) -> Option<Token> {
        match self.stream.pop() {
            Some(tk) => {
                if tk.kind == kind {
                    return Some(tk);
                } else {
                    return None;
                }
            }
            None => None,
        }
    }

    pub(in crate::ast) fn peek(&mut self) -> Option<&Token> {
        self.stream.last()
    }

    pub(in crate::ast) fn peek_as(&mut self, kind: TokenKind) -> Option<&Token> {
        match self.stream.last() {
            Some(tk) => {
                if tk.kind == kind {
                    return Some(tk);
                } else {
                    return None;
                }
            }
            None => None,
        }
    }

    pub(in crate::ast) fn span_now(&self) -> Range<usize> {
        self.span.clone()
    }
}
