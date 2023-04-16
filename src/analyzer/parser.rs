use std::{collections::VecDeque, rc::Rc};

use super::{
    syntax_tree::{SyntaxNode, SyntaxTree},
    token::{self, Token},
};
use anyhow::{anyhow, Result};

pub fn parse<T: IntoIterator<Item = Token>>(token_stream: T) -> Result<SyntaxTree> {
    let mut queue = reverse_polish(token_stream)?;

    let token = match queue.pop_back() {
        Some(t) => t,
        None => return Err(anyhow!("empty expression")),
    };

    let syntax_tree = SyntaxTree::with_root(SyntaxNode::new(token).into());
    let mut cursor = syntax_tree.get_root();

    while cursor.is_some() {
        // Option::clone will trigger Rc::clone
        let node = Option::clone(&cursor).unwrap();

        if matches!(node.value.kind, token::Kind::UnaryOperator(_)) && node.get_children().len() == 1 {
            cursor = node.get_parent_clone();
            continue;
        }

        if matches!(node.value.kind, token::Kind::BinaryOperator(_)) && node.get_children().len() == 2 {
            cursor = node.get_parent_clone();
            continue;
        }

        let token = match queue.pop_back() {
            Some(t) => t,
            None => break,
        };

        match token.kind {
            token::Kind::Number(_) => {
                let new_node = Rc::new(SyntaxNode::new(token));

                SyntaxTree::add_child(node, new_node);
            }

            token::Kind::Func(_) | token::Kind::UnaryOperator(_) | token::Kind::BinaryOperator(_) => {
                let new_node = Rc::new(SyntaxNode::new(token));
                cursor = Some(Rc::clone(&new_node));

                SyntaxTree::add_child(node, new_node);
            }

            token::Kind::Delimeter(token::Delim::FuncArgs) => {
                cursor = node.get_parent();
            }

            _ => {}
        }
    }

    Ok(syntax_tree)
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
                    stack.pop_front();
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
