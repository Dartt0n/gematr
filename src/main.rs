mod analyzer;
mod math;
use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let expr = fs::read_to_string("examples/expr7.rth").expect("failed to read file");
    println!("Input Expression:\n\t{}", &expr);

    let tokens = analyzer::lexer::tokenize(expr.chars())?;

    println!("Tokens:");
    for t in &tokens {
        println!("\t{:?}", t.kind);
    }

    let rpn = analyzer::parser::reverse_polish(tokens.into_iter())?;

    println!("Reverse Polish:");
    for t in &rpn {
        println!("\t{:?}", t.kind);
    }

    // https://cs.lmu.edu/~ray/notes/compilerarchitecture/

    Ok(())
}
