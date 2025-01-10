use std::{f32::consts::E, fmt::Display, ops::Range};

use crate::{
    ast::{AbstractSyntaxTree, Expr, Operator, OperatorKind},
    scanner::Scanner,
    token::{Token, TokenKind},
};

#[derive(Debug)]
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

#[derive(Debug)]
pub(crate) struct Parser {
    pub errors: Vec<ParseError>,
    scanner: Scanner,
    tree: AbstractSyntaxTree,
    stack: Vec<Expr>,
    holding_stack: Vec<Operator>,
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

    pub(crate) fn parse(&mut self) {
        loop {
            self.parse_expr();

            // Check for EOF
            match self.stack.last() {
                Some(e) => match e {
                    Expr::EndOfFile { code: _ } => break,
                    _ => continue,
                },
                None => continue,
            }
        }
    }

    fn parse_expr(&mut self) {
        if let Some(tk) = self.scanner.scan_next() {
            match tk.kind {
                TokenKind::Dot => self.parse_dot(),
                TokenKind::Ident(s) => self.parse_symbol(s),
                TokenKind::Number(s) => self.parse_number(s),
                TokenKind::EOF => self.end(),
                _ => self.parse_op(&tk.kind),
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

        // Push the IndexInto EXPR to the stack
        self.stack.push(Expr::IndexInto {
            item: Box::new(lhs),
            index: Box::new(rhs),
        });
    }

    fn parse_op(&mut self, tk: &TokenKind) {
        match Operator::fromtk(tk) {
            Some(op) => self.push_op(op),
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

    fn push_op(&mut self, newop: Operator) {
        match newop.op_kind {
            OperatorKind::ParOpen => {
                // Put the new op on the holding stack
                self.holding_stack.push(newop);
                return; // early return
            }
            OperatorKind::ParClose => {
                println!("Draining the stack until (");
                self.drain_until_paropen();
                return; // early return
            }
            _ => {}
        }

        match self.holding_stack.last() {
            Some(op) => {
                // If newop has a lower precendence than most recent stack op
                if newop.precedence < op.precedence && op.op_kind != OperatorKind::ParOpen {
                    // Pop the current operator and put it on the stack
                    let op = unsafe { self.holding_stack.pop().unwrap_unchecked() };
                    self.stack.push(Expr::Operator { op });

                    // Put the new op on the holding stack
                    self.holding_stack.push(newop);
                    return; // early return
                }
            }
            None => {}
        }
        // Put the new op on the holding stack
        self.holding_stack.push(newop);
    }

    fn drain_until_paropen(&mut self) {
        loop {
            dbg!(&self.holding_stack);
            if let Some(op) = self.holding_stack.pop() {
                if op.op_kind == OperatorKind::ParOpen {
                    eprintln!("( found");
                    break;
                } else {
                    self.stack.push(Expr::Operator { op });
                    continue;
                }
            }
            break;
        }
    }

    /// Calls `parse_expr()` and returns the last thing added to the stack, if it exists
    /// Will return none if self.parse() does not parse anything or if the stack is empty
    fn parse_and_get(&mut self) -> Option<Expr> {
        self.parse();
        self.stack.pop()
    }

    fn end(&mut self) {
        // Drain the stack in reverse order ops were added
        self.holding_stack.reverse();
        self.holding_stack.drain(0..).for_each(|op| {
            self.stack.push(Expr::Operator { op });
        });

        // Push EOF node
        self.stack.push(Expr::EndOfFile { code: 0 });
    }
}
