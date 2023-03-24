use std::collections::VecDeque;

use super::{
    syntax_tree::ArenaSyntaxTree,
    token::{self, Token},
};
use anyhow::{anyhow, Result};

pub fn parse<T: IntoIterator<Item = Token>>(token_stream: T) -> Result<ArenaSyntaxTree> {
    let mut queue = reverse_polish(token_stream)?;

    let token = match queue.pop_back() {
        Some(t) => t,
        None => return Err(anyhow!("empty expression")),
    };

    let mut arena = ArenaSyntaxTree::new(token);
    let mut current_index = 0;

    while arena.get(current_index).is_some() {
        let current_node = arena.get(current_index).unwrap();

        if matches!(current_node.value.kind, token::Kind::UnaryOperator(_)) {
            if current_node.children.len() > 1 {
                return Err(anyhow!("invalid number of arguments for unary operator"));
            }
            if current_node.children.len() == 1 {
                current_index = match current_node.parent {
                    Some(i) => i,
                    None => break,
                };
                continue;
            }
        }

        if matches!(current_node.value.kind, token::Kind::BinaryOperator(_)) {
            if current_node.children.len() > 2 {
                return Err(anyhow!("invalid number of arguments for binary operator"));
            }
            if current_node.children.len() == 2 {
                current_index = match current_node.parent {
                    Some(i) => i,
                    None => break,
                };
                continue;
            }
        }

        let token = match queue.pop_back() {
            Some(t) => t,
            None => break,
        };

        match token.kind {
            token::Kind::Number(_) => {
                arena.insert(current_index, token);
            }

            token::Kind::Func(_) | token::Kind::UnaryOperator(_) | token::Kind::BinaryOperator(_) => {
                current_index = arena.insert(current_index, token);
            }

            token::Kind::Delimeter(token::Delim::FuncArgs) => match current_node.parent {
                Some(i) => current_index = i,
                None => break,
            },

            _ => {}
        }
    }

    Ok(arena)
}

pub fn reverse_polish<T: IntoIterator<Item = Token>>(token_stream: T) -> Result<VecDeque<Token>> {
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
