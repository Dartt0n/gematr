use super::lexer::{Associativity, Token, TokenKind};
use std::collections::VecDeque;
use std::vec;

pub fn reorder(tokens: Vec<Token>) -> Result<VecDeque<Token>, ()> {
    let mut stack = vec![]; // stack: push, pop, last
    let mut queue = VecDeque::new();

    for token in tokens {
        match token.kind {
            TokenKind::Literal => queue.push_back(token),
            TokenKind::Function => stack.push(token),
            TokenKind::Operator => {
                while stack.last().map_or(false, |t| {
                    t.precedence > token.precedence
                        || t.precedence == token.precedence
                            && token.associativity == Associativity::Left
                }) {
                    queue.push_back(stack.pop().unwrap())
                }
                stack.push(token)
            }
            TokenKind::Parenthesis if token.value.as_str() == "(" => stack.push(token),
            TokenKind::Parenthesis if token.value.as_str() == ")" => {
                while stack.last().map_or(false, |t| {
                    t.kind != TokenKind::Parenthesis && t.value.as_str() != "("
                }) {
                    queue.push_back(stack.pop().unwrap())
                }

                if stack.last().map_or(false, |t| {
                    t.kind == TokenKind::Parenthesis && t.value.as_str() == "("
                }) {
                    stack.pop(); // discard left parenthesis
                } else {
                    return Err(()); // todo: handle error
                }

                if stack
                    .last()
                    .map_or(false, |t| t.kind == TokenKind::Function)
                {
                    queue.push_back(stack.pop().unwrap())
                }
            }

            _ => {}
        }
    }

    while !stack.is_empty() {
        if stack.last().map_or(false, |t| {
            t.kind == TokenKind::Parenthesis && t.value.as_str() == "("
        }) {
            return Err(()); // todo handle error
        }

        queue.push_back(stack.pop().unwrap())
    }

    return Ok(queue);
}
