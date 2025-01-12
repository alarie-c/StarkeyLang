use std::{fmt::Display, ops::Range};

use crate::{
    ast::{Expr, ExprKind, Operator, OperatorKind},
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
    stream: Vec<Token>,
    span: Range<usize>,

    /// This is effectively the holding
    /// stack for operators
    op_stack: Vec<Operator>,

    /// This is the output for the Parser
    expr_stack: Vec<Expr>,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Self {
            errors: vec![],
            span: 0..0,
            stream: tokens,
            op_stack: vec![],
            expr_stack: vec![],
        }
    }

    pub(crate) fn parse_expressions(&mut self) {
        dbg!(&self);

        while !self.stream.is_empty() {
            self.parse_expr();
        }
    }

    fn parse_expr(&mut self) {
        if let Some(tk) = self.next() {
            println!("PARSING: ");
            dbg!(&tk);
            match tk.kind {
                TokenKind::Function => self.parse_function_signature(),
                TokenKind::End => self.parse_marker(ExprKind::MarkerEnd),
                TokenKind::Ident(s) => self.parse_symbol(s),
                TokenKind::Number(s) => self.parse_number(s),
                TokenKind::EOF => self.end(),
                TokenKind::Comma => self.parse_expr(),
                _ => self.parse_op(&tk.kind),
            }
        }
    }

    fn parse_parameters(&mut self) {
        println!("current:");
        dbg!(&self);

        let mut params: Vec<Box<Expr>> = vec![];
        loop {
            // Check to see if the next thing is a close par
            if self.peek().is_some_and(|tk| tk.kind == TokenKind::ParClose) {
                self.next(); // consume that ^
                break;
            }

            self.parse_expr();
            match self.last() {
                Some(e) => match e.kind {
                    ExprKind::TypedSymbol { stype: _, sname: _ } => unsafe {
                        params.push(Box::new(self.pop().unwrap_unchecked()));
                    },
                    _ => break,
                },
                None => panic!("Expected function parameters after function name definintion"),
            }
        }
        let expr = self.expr(ExprKind::Parameters { params });
        self.push(expr);
    }

    fn parse_function_signature(&mut self) {
        // Get the name of the function
        let fsymbol = self
            .parse_and_get()
            .unwrap_or_else(|| panic!("Expected a valid symbol for function signature"));

        // Expect the next thing to be a open parenthesis
        if !self.peek().is_some_and(|tk| tk.kind == TokenKind::ParOpen) {
            panic!("Expected '(' after function symbol definition");
        }
        self.next(); // consume that ^
        self.parse_parameters();

        // Take parameters off the stack
        // unsafe unwrap because parse_params will always push a params node, even if it's empty
        let fparams = unsafe { self.pop().unwrap_unchecked() };

        // Expect the next thing to be a colon
        let mut freturn: Option<Box<Expr>> = None;
        if self.peek().is_some_and(|tk| tk.kind == TokenKind::Colon) {
            self.next(); // consume that ^
            freturn = Some(Box::new(self.parse_and_get().unwrap_or_else(|| {
                panic!("Expected valid symobl after function return definition");
            })));
        }

        // Assemble the function signature and push it to the AST
        let expr = self.expr(ExprKind::FunctionSignature {
            fsymbol: Box::new(fsymbol),
            fparams: Box::new(fparams),
            freturn,
        });
        self.push(expr);
    }

    fn parse_op(&mut self, tk: &TokenKind) {
        match Operator::from_token(tk) {
            Some(op) => self.push_op(op),
            None => panic!("Tried to make an operator from a non-operator token!"),
        }
    }

    fn parse_symbol(&mut self, s: String) {
        let item = self.expr(ExprKind::Symbol { name: s });

        // We already have the symbol name, s, and it's token has been
        // consumed. We want to check to make sure this symbol isn't being indexed into
        if let Some(tk) = self.peek() {
            match tk.kind {
                TokenKind::Dot => {
                    // Consume the dot
                    let _ = self.next();

                    // Parse and get the next thing
                    let index = self.parse_and_get().unwrap_or_else(|| {
                        panic!("Dot found, expected a valid symbol to index into this symobl");
                    });

                    // Push this to the stack
                    let expr = self.expr(ExprKind::IndexInto {
                        item: Box::new(item),
                        index: Box::new(index),
                    });
                    self.push(expr);
                    return; // early return
                }
                TokenKind::Colon => {
                    // Consume the colon
                    let _ = self.next();

                    // Parse and get the type
                    let stype = self.parse_and_get().unwrap_or_else(|| {
                        panic!("Dot found, expected a valid symbol to index into this symobl");
                    });

                    // Push this to the stack
                    let expr = self.expr(ExprKind::TypedSymbol {
                        stype: Box::new(stype),
                        sname: Box::new(item),
                    });
                    self.push(expr);
                    return; // early return
                }
                _ => {}
            }
        }
        self.push(item);
    }

    fn parse_number(&mut self, n: String) {
        match n.parse::<f64>() {
            Ok(v) => {
                let expr = self.expr(ExprKind::Number { value: v });
                self.push(expr);
            }
            Err(_) => panic!("Error parsing number!"),
        }
    }

    fn parse_marker(&mut self, marker: ExprKind) {
        let expr = self.expr(marker);
        self.push(expr);
    }

    /// Calls `parse_expr()` and returns the last thing added to the stack, if it exists
    /// Will return none if self.parse() does not parse anything or if the stack is empty
    fn parse_and_get(&mut self) -> Option<Expr> {
        self.parse_expr();
        self.pop()
    }

    fn push_op(&mut self, newop: Operator) {
        match newop.op_kind {
            OperatorKind::ParOpen => {
                self.op_stack.push(newop);
                return; // early return
            }
            OperatorKind::ParClose => {
                self.drain_paren_group();
                return; // early return
            }
            _ => {}
        }

        match self.op_stack.last() {
            Some(op) => {
                // If newop has a lower precendence than most recent stack op
                if newop.precedence < op.precedence && op.op_kind != OperatorKind::ParOpen {
                    // Pop the current operator and push it to the AST
                    let op = unsafe { self.op_stack.pop().unwrap_unchecked() };
                    let expr = self.expr(ExprKind::Operator { op });
                    self.push(expr);

                    // Put the new op on the holding stack
                    self.op_stack.push(newop);
                    return; // early return
                }
            }
            None => {}
        }

        // Put the new op on the holding stack
        self.op_stack.push(newop);
    }

    fn drain_paren_group(&mut self) {
        loop {
            if let Some(op) = self.op_stack.pop() {
                if op.op_kind == OperatorKind::ParOpen {
                    break;
                } else {
                    let expr: Expr = self.expr(ExprKind::Operator { op });
                    self.push(expr);
                    continue;
                }
            }
            break;
        }
    }

    fn expr(&mut self, kind: ExprKind) -> Expr {
        Expr {
            kind,
            span: self.span.clone(),
        }
    }

    /// Converts the given EXPR to a node
    /// and pushes it to the AST
    fn push(&mut self, expr: Expr) {
        self.expr_stack.push(expr);
    }

    fn pop(&mut self) -> Option<Expr> {
        match self.expr_stack.pop() {
            Some(e) => {
                self.span.start = e.span.start;
                return Some(e);
            }
            None => None,
        }
    }

    fn last(&mut self) -> Option<&Expr> {
        self.expr_stack.last()
    }

    fn peek(&mut self) -> Option<&Token> {
        self.stream.last()
    }

    fn next(&mut self) -> Option<Token> {
        self.stream.pop()
    }

    fn end(&mut self) {
        // Drain the stack in reverse order ops were added
        self.op_stack.reverse();
        while let Some(op) = self.op_stack.pop() {
            let expr: Expr = self.expr(ExprKind::Operator { op });
            self.push(expr);
        }

        // Push EOF node
        let expr = self.expr(ExprKind::EndOfFile { code: 0 });
        self.push(expr);
    }
}
