mod math;
mod parser;
use anyhow::Result;

fn main() -> Result<()> {
    let expr = "5 + 5.6 + 5..6";
    println!("Input Expression:\n{}", &expr);

    let tokens = parser::lexer::tokenize(expr)?;
    println!("Tokens:");
    for token in tokens.iter() {
        print!("{} ", token.value);
    }

    Ok(())
}
