use std::fs;

use parser::{ParseError, Parser};
use scanner::Scanner;
use token::TokenKind;

mod ast;
mod parser;
mod scanner;
mod token;

const SOURCE_PATH: &str = "main.sk";

fn main() {
    let src = fs::read_to_string(SOURCE_PATH).expect("Error reading source file");

    // Construct the scanner and tokenize the source file
    let mut scanner = Scanner::new(&src);
    let mut tokens = scanner.scan();
    tokens.reverse();
    println!("{tokens:?}");

    // Construct the parser and feed it the tokens from the scanner
    let mut parser = Parser::new(tokens);
    parser.parse_expressions();
    dbg!(&parser);
}
