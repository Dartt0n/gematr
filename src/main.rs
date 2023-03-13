mod math;
mod parser;
use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let expr = fs::read_to_string("examples/expr7.rth").expect("failed to read file");

    println!("Input Expression:\n{}", &expr);

    let tokens = parser::lexer::tokenize(&expr)?;

    // TODO: Refactor (again)
    parser::parse(tokens);

    Ok(())
}
