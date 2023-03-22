use std::collections::VecDeque;

use super::{
    syntax_tree::SyntaxTree,
    token::{self, Token},
};
use anyhow::{anyhow, Result};

pub fn build_tree<TokenIter>(token_stream: TokenIter) -> Result<SyntaxTree>
where
    TokenIter: Iterator<Item = Token>,
{
    let queue = reverse_polish(token_stream)?;

    todo!("build tree from reverse polish notation")
}

pub fn reverse_polish<TokenIter>(token_stream: TokenIter) -> Result<VecDeque<Token>>
where
    TokenIter: Iterator<Item = Token>,
{
    let mut stack = VecDeque::<Token>::new();
    let mut queue = VecDeque::<Token>::new();

    for token in token_stream {
        match token.kind {
            token::Kind::Number(_) => queue.push_back(token),
            token::Kind::Func(_) => stack.push_front(token),
            token::Kind::Delimeter(token::Delim::FuncArgs) => queue.push_back(token),
            token::Kind::Parenthesis(token::Paren::Open) => stack.push_front(token),

            token::Kind::Parenthesis(token::Paren::Close) => {
                while on_top(&stack, |t| t.kind != token::Kind::Parenthesis(token::Paren::Open)) {
                    queue.push_back(stack.pop_front().unwrap());
                }

                if on_top(&stack, |t| t.kind == token::Kind::Parenthesis(token::Paren::Open)) {
                    stack.pop_front(); // discard left parenthesis
                } else {
                    return Err(anyhow!("unmatched parenthesis in the token stream"));
                }
            }

            token::Kind::BinaryOperator(_) | token::Kind::UnaryOperator(_) => {
                while on_top(&stack, |t| {
                    t.prec > token.prec || t.prec == token.prec && token.assoc == token::Associativity::Left
                }) {
                    queue.push_back(stack.pop_front().unwrap());
                }
                stack.push_front(token);
            }

            token::Kind::Delimeter(token::Delim::Comma) => {
                while on_top(&stack, |t| {
                    t.kind != token::Kind::Parenthesis(token::Paren::Open) && t.prec > token.prec
                }) {
                    queue.push_back(stack.pop_front().unwrap());
                }
            }
        }
    }

    while !stack.is_empty() {
        queue.push_back(stack.pop_front().unwrap());
    }

    return Ok(queue);
}

fn on_top<F>(stack: &VecDeque<Token>, condition: F) -> bool
where
    F: Fn(&Token) -> bool,
{
    stack.front().map_or(false, condition)
}
