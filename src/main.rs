use std::fs;

mod parser;

fn main() {
    let content = fs::read_to_string("examples/expr1.rth").expect("Failed to read file");
    let tokens = parser::lexer::tokenize(content).unwrap();
    dbg!(parser::shunting_yard::reorder(tokens));
}
