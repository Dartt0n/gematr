mod analyzer;
mod math;
use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let expr = fs::read_to_string("examples/expr7.rth").expect("failed to read file");
    println!("Input Expression:\n{}", &expr);

    match analyzer::lexer::tokenize(expr.chars()) {
        Ok(tokens) => {
            for t in tokens {
                print!("{:?} ", t.kind);
            }
        }
        Err(err) => println!("Errors: {}", err),
    }

    // https://cs.lmu.edu/~ray/notes/compilerarchitecture/

    Ok(())
}
