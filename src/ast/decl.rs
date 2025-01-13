use std::ops::Range;

use crate::token::TokenKind;

use super::expr::{Expr, ExprKind};
use super::parser::Parser;
use super::stmt::Stmt;

#[derive(Debug)]
pub(crate) struct Decl {
    pub kind: DeclKind,
    pub span: Range<usize>,
}

#[derive(Debug)]
pub(crate) enum DeclKind {
    Function {
        symbol: Expr,
        params: Vec<Expr>,
        returns: Option<Expr>,
    },
    GlobalValue {
        assignment: Expr,
    },
    ImportDirective {
        symbols: Vec<Expr>,
    },
}

impl Parser {
    pub(in crate::ast) fn parse_decl(&mut self) -> Option<Decl> {
        // Consume tokens until we find something that posits a declaration
        while let Some(tk) = self.next() {
            match &tk.kind {
                TokenKind::Function => return self.parse_function(),
                _ => continue,
            }
        }
        return None;
    }

    fn parse_function(&mut self) -> Option<Decl> {
        // Start by getting the name
        let symbol = self.parse_expr().unwrap_or_else(|| {
            panic!("Expected an expression following `function` token!");
        });

        // Assert that symbol is actually a symbol
        match &symbol.kind {
            ExprKind::Symbol { ident: _ }
            | ExprKind::TypedSymbol {
                symbol: _,
                stype: _,
            } => {}
            _ => panic!("Expected the next expression following `function` token to be `symbol`!"),
        }

        // Consume a token and expect an open parenthesis
        match self.next_assert(TokenKind::ParOpen) {
            Some(_) => {}
            None => panic!("Expected an open parentheses following function symbol"),
        }

        // Take typed expressions until we get a par close
        let mut params: Vec<Expr> = vec![];
        while let None = self.peek_assert(TokenKind::ParClose) {
            // Parse this expression
            let p = self.parse_expr().unwrap_or_else(|| {
                panic!("Expected a valid expression following open par for function params");
            });

            // Assert that p is a typed symbol
            match &p.kind {
                ExprKind::TypedSymbol {
                    symbol: _,
                    stype: _,
                } => {}
                _ => panic!(
                    "Expected a typed symbol expression following open par for function params"
                ),
            }

            // Add this to the params list
            params.push(p);
        }

        // Consume the closed parenthesis
        let _ = self.next();

        // Look for a colon token so we can determine if this function is typed or not
        let mut returns: Option<Expr> = None;
        match self.next_assert(TokenKind::Colon) {
            Some(_) => {
                // Start by getting the name of the type
                returns = Some(self.parse_expr().unwrap_or_else(|| {
                    panic!("Expected an expression following `function` token!");
                }));

                // Assert that returns is actually a symbol
                match &returns.as_ref().unwrap().kind {
                    ExprKind::Symbol { ident: _ }
                    | ExprKind::TypedSymbol {
                        symbol: _,
                        stype: _,
                    } => {}
                    _ => panic!("Expected the return type of a function to be `symbol`!"),
                }
            }
            None => {}
        }

        // Build out the function declaration
        return Some(Decl {
            kind: DeclKind::Function {
                symbol,
                params,
                returns,
            },
            span: self.span_now(),
        });
    }
}
