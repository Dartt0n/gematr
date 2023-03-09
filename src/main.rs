mod math;
mod parser;

use std::fs;

fn main() {
    let content = fs::read_to_string("examples/expr3.rth").expect("Failed to read file");
    println!("Input Expression:\n{}", &content);

    let tokens = parser::lexer::tokenize(content).unwrap();
    let rpn = parser::shunting_yard::reorder(tokens);

    match rpn {
        Err(e) => println!("Error: {}", e),
        Ok(v) => {
            println!("Tokenization in Reverse Polish Notation:");
            for i in v.iter() {
                print!("{:?} ", i.value);
            }
            println!()
        }
    }
}
