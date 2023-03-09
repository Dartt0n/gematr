mod math;
mod parser;

use std::fs;

fn main() {
    let content = fs::read_to_string("examples/expr6.rth").expect("Failed to read file");
    println!("Input Expression:\n{}", &content);

    let tokens = parser::lexer::tokenize(content).unwrap();
    let rpn = parser::shunting_yard::reorder(tokens).unwrap();

    println!("Output Reverse Polish Notation:");
    for i in rpn.iter() {
        print!("{} ", i.value);
    }
    println!()
}
