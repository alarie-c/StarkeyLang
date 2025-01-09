use crate::{ast::AbstractSyntaxTree, scanner::Scanner, token::Token};

pub(crate) struct Parser {
    scanner: Scanner,
    tree: AbstractSyntaxTree,
    stack: Vec<Token>,
    holding_stack: Vec<Token>,
}

impl Parser {
    pub(crate) fn new(source: &String) -> Self {
        Self {
            scanner: Scanner::new(source),
            tree: AbstractSyntaxTree::new(),
            stack: vec![],
            holding_stack: vec![],
        }
    }

    pub(crate) fn parse(&mut self) {}
}
