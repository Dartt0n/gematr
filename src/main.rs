mod analyzer;
mod math;
use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let expr = fs::read_to_string("examples/expr8.gm").expect("failed to read file");
    println!("Input Expression:\n\t{}", &expr);

    let tokens = analyzer::lexer::tokenize(expr.chars())?;
    let tree_arena = analyzer::parser::parse(tokens)?;

    println!("Syntax Tree:\n{}", tree_arena);

    // https://cs.lmu.edu/~ray/notes/compilerarchitecture/

    Ok(())
}
