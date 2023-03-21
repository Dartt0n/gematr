use super::{syntax_tree::SyntaxTree, token::Token};
use anyhow::Result;

pub fn build_tree<TokenIter>(token_stream: TokenIter) -> Result<SyntaxTree>
where
    TokenIter: Iterator<Item = Token>,
{
    todo!()
}
