mod analyzer;
mod interpreter;
use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let expr = fs::read_to_string("examples/expr12.gm").expect("failed to read file");
    println!("Input Expression:\n\t{}", &expr);

    let tokens = analyzer::lexer::tokenize(expr.chars())?;
    let tree = analyzer::parser::parse(tokens)?;

    println!("Syntax Tree:\n{}", tree);

    let tree = analyzer::semantic_analyzer::opimize(tree)?;

    println!("Optimized Syntax Tree:\n{}", tree);
    // https://cs.lmu.edu/~ray/notes/compilerarchitecture/

    println!("Result: {}", interpreter::interprete(tree));
    Ok(())
}
