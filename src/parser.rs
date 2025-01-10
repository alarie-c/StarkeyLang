use std::{f32::consts::E, fmt::Display, ops::Range};

use crate::{
    ast::{AbstractSyntaxTree, Expr, Operator},
    scanner::Scanner,
    token::{Token, TokenKind},
};

pub(crate) struct ParseError {
    pub pos: Range<usize>,
    pub msg: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pos = format!("[PARSE_ERROR] at [{}-{}]", self.pos.start, self.pos.end);
        write!(f, "{pos}\n\t{}", self.msg)
    }
}

pub(crate) struct Parser {
    pub errors: Vec<ParseError>,
    scanner: Scanner,
    tree: AbstractSyntaxTree,
    stack: Vec<Expr>,
    holding_stack: Vec<Expr>,
}

impl Parser {
    pub(crate) fn new(source: &String) -> Self {
        Self {
            errors: vec![],
            scanner: Scanner::new(source),
            tree: AbstractSyntaxTree::new(),
            stack: vec![],
            holding_stack: vec![],
        }
    }

    pub(crate) fn parse(&mut self) {}

    fn parse_expr(&mut self) {
        if let Some(tk) = self.scanner.scan_next() {
            match tk.kind {
                TokenKind::Dot => self.parse_dot(),
                TokenKind::Ident(s) => self.parse_symbol(s),
                TokenKind::Number(s) => self.parse_number(s),
                TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash => {
                    self.parse_op(&tk.kind)
                }
                _ => panic!("Unexpected token!"),
            }
        }
    }

    fn parse_dot(&mut self) {
        let lhs = self.stack.pop().unwrap_or_else(|| {
            panic!("Expected an LHS expression for MemberAccess expression!");
        });
        let rhs = self.parse_and_get().unwrap_or_else(|| {
            panic!("Expected an RHS expression for MemberAccess expression!");
        });

        // Push the AccessMember EXPR to the stack
        self.stack.push(Expr::AccessMember {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        });
    }

    fn parse_op(&mut self, tk: &TokenKind) {
        match Operator::fromtk(tk) {
            Some(op) => self.stack.push(Expr::Operator { op }),
            None => panic!("Tried to make an operator from a non-operator token!"),
        }
    }

    fn parse_symbol(&mut self, n: String) {
        self.stack.push(Expr::Symbol { name: n });
    }

    fn parse_number(&mut self, n: String) {
        match n.parse::<f64>() {
            Ok(v) => self.stack.push(Expr::Number { value: v }),
            Err(_) => panic!("Error parsing number!"),
        }
    }

    // Calls `parse_expr()` and checks the last thing added to the stack to see if it was a symbol
    // fn expect_symbol(&mut self) -> bool {
    //     self.parse_expr();
    //     match self.stack.last() {
    //         Some(e) => match e {
    //             Expr::Symbol { name: _ } => true,
    //             _ => false,
    //         },
    //         None => false,
    //     }
    // }

    /// Calls `parse_expr()` and returns the last thing added to the stack, if it exists
    /// Will return none if self.parse() does not parse anything or if the stack is empty
    fn parse_and_get(&mut self) -> Option<Expr> {
        self.parse();
        self.stack.pop()
    }
}
