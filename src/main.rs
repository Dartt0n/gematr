mod math;
mod parser;

use std::fs;

fn main() {
    let content = fs::read_to_string("examples/expr3.rth").expect("Failed to read file");
    let tokens = parser::lexer::tokenize(content).unwrap();
    let rpn = parser::shunting_yard::reorder(tokens).unwrap();
    dbg!(&rpn);
}
