use std::fs;

mod parser;

fn main() {
    // Expression 1:
    // Expression 2:
    // Expression 3:

    // Task: Build Abstract Syntax Tree
    // Goal: Check syntax, report errors, ready-to-go schema of expression

    let content = fs::read_to_string("examples/expr3.rth").expect("Failed to read file");
    dbg!(parser::lexer::tokenize(content));
}