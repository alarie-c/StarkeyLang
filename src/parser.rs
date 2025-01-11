use std::{f32::consts::E, fmt::Display, ops::Range};

use crate::{
    ast::{AbstractSyntaxTree, Expr, Node, Operator, OperatorKind},
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
    stream: Vec<Token>,
    span: Range<usize>,

    /// This is effectively the holding
    /// stack for operators
    stack: Vec<Operator>,

    /// This is the output for the Parser
    tree: AbstractSyntaxTree,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Self {
            errors: vec![],
            span: 0..0,
            stream: tokens,
            stack: vec![],
            tree: AbstractSyntaxTree::new(),
        }
    }

    pub(crate) fn parse(&mut self) {
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
                Some(n) => match n.expr {
                    Expr::TypedSymbol { stype: _, sname: _ } => unsafe {
                        params.push(Box::new(self.pop().unwrap_unchecked()));
                    },
                    _ => break,
                },
                None => panic!("Expected function parameters after function name definintion"),
            }
        }
        self.push(Expr::Parameters { params });
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
        self.push(Expr::FunctionSignature {
            fsymbol: Box::new(fsymbol),
            fparams: Box::new(fparams),
            freturn,
        });

        // TODO: Parse the function body here I guess
    }

    fn parse_op(&mut self, tk: &TokenKind) {
        match Operator::from_token(tk) {
            Some(op) => self.push_op(op),
            None => panic!("Tried to make an operator from a non-operator token!"),
        }
    }

    fn parse_symbol(&mut self, s: String) {
        let item = Expr::Symbol { name: s };

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
                    self.push(Expr::IndexInto {
                        item: Box::new(item),
                        index: Box::new(index),
                    });
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
                    self.push(Expr::TypedSymbol {
                        stype: Box::new(stype),
                        sname: Box::new(item),
                    });
                    return; // early return
                }
                _ => {}
            }
        }
        self.push(item);
    }

    fn parse_number(&mut self, n: String) {
        match n.parse::<f64>() {
            Ok(v) => self.push(Expr::Number { value: v }),
            Err(_) => panic!("Error parsing number!"),
        }
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
                self.stack.push(newop);
                return; // early return
            }
            OperatorKind::ParClose => {
                self.drain_paren_group();
                return; // early return
            }
            _ => {}
        }

        match self.stack.last() {
            Some(op) => {
                // If newop has a lower precendence than most recent stack op
                if newop.precedence < op.precedence && op.op_kind != OperatorKind::ParOpen {
                    // Pop the current operator and push it to the AST
                    let op = unsafe { self.stack.pop().unwrap_unchecked() };
                    self.push(Expr::Operator { op });

                    // Put the new op on the holding stack
                    self.stack.push(newop);
                    return; // early return
                }
            }
            None => {}
        }

        // Put the new op on the holding stack
        self.stack.push(newop);
    }

    fn drain_paren_group(&mut self) {
        loop {
            if let Some(op) = self.stack.pop() {
                if op.op_kind == OperatorKind::ParOpen {
                    break;
                } else {
                    self.push(Expr::Operator { op });
                    continue;
                }
            }
            break;
        }
    }

    /// Converts the given EXPR to a node
    /// and pushes it to the AST
    fn push(&mut self, expr: Expr) {
        self.tree.push_node(Node {
            expr,
            span: self.span.clone(),
        });
    }

    fn pop(&mut self) -> Option<Expr> {
        match self.tree.pull_node() {
            Some(n) => {
                self.span.start = n.span.start;
                return Some(n.expr);
            }
            None => None,
        }
    }

    fn last(&mut self) -> Option<&Node> {
        self.tree.last_node()
    }

    fn peek(&mut self) -> Option<&Token> {
        self.stream.last()
    }

    fn next(&mut self) -> Option<Token> {
        self.stream.pop()
    }

    fn end(&mut self) {
        // Drain the stack in reverse order ops were added
        self.stack.reverse();
        while let Some(op) = self.stack.pop() {
            self.push(Expr::Operator { op });
        }

        // Push EOF node
        self.push(Expr::EndOfFile { code: 0 });
    }
}
