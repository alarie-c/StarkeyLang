use std::fs;

use parser::Parser;
use scanner::Scanner;
use token::TokenKind;

mod ast;
mod parser;
mod scanner;
mod token;

const SOURCE_PATH: &str = "main.sk";

fn main() {
    let src = fs::read_to_string(SOURCE_PATH).expect("Error reading source file");

    // let mut scanner = Scanner::new(&src);
    // while let Some(t) = scanner.scan_next() {
    //     println!("{t}");
    //     if t.kind == TokenKind::EOF {
    //         break;
    //     }
    // }

    let mut parser = Parser::new(&src);
    parser.parse();
}
