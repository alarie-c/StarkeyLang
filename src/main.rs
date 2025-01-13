use std::fs;

use scanner::scanner::Scanner;

mod scanner;
mod parser;

const SOURCE_PATH: &str = "main.sk";

fn main() {
    let src = fs::read_to_string(SOURCE_PATH).expect("Error reading source file");

    // Construct the scanner and tokenize the source file
    let mut scanner = Scanner::new(&src);
    let mut tokens = scanner.scan();
    tokens.iter().for_each(|x| println!("{x}"));
    tokens.reverse(); // Reverse this so when we call pop() the first thing off the stack is the first token we put on it
}
