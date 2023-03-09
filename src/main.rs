mod math;
mod parser;

use crate::parser::syntax_tree::SyntaxTree;
use std::fs;

fn main() {
    let content = fs::read_to_string("examples/expr3.rth").expect("Failed to read file");
    let tokens = parser::lexer::tokenize(content).unwrap();
    let rpn = parser::shunting_yard::reorder(tokens).unwrap();

    for i in rpn.iter() {
        println!("\"{}{}{}\" [label=\"{}\"]", i.value, i.line, i.column, i.value);
    }

    dbg!(SyntaxTree::build(rpn));
}
