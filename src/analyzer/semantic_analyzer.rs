use super::syntax_tree::SyntaxTree;
use anyhow::Result;

pub fn process(_syntax_tree: &SyntaxTree) -> Result<()> {
    todo!("Traveral tree")
}

pub fn opimize(syntax_tree: &mut SyntaxTree) -> Result<&mut SyntaxTree> {
    process(&syntax_tree)?;

    // TODO: eliminate unary pluses
    // TODO: put unary minuses inside numbers
    // TODO: eliminate double unary minuses

    Ok(syntax_tree)
}
