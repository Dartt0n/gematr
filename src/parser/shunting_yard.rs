use super::lexer::{Associativity, Token, TokenKind};
use anyhow::{anyhow, Result};
use std::collections::VecDeque;

fn on_top<F>(stack: &VecDeque<Token>, func: F) -> bool
where
    F: Fn(&Token) -> bool,
{
    stack.front().map_or(false, func)
}

pub fn reorder(tokens: Vec<Token>) -> Result<VecDeque<Token>> {
    let mut stack = VecDeque::new();
    let mut queue = VecDeque::new();

    for token in tokens {
        match token.kind {
            TokenKind::Literal => queue.push_back(token),

            TokenKind::Function => {
                queue.push_back(Token::delimiter("#".to_string(), 0, 0));
                stack.push_front(token);
            }

            TokenKind::Separator => {
                while on_top(&stack, |t| t.value != "(" && t.precedence >= token.precedence) {
                    queue.push_back(stack.pop_front().unwrap())
                }
            }

            TokenKind::Operator => {
                while on_top(&stack, |t| {
                    t.precedence > token.precedence
                        || t.precedence == token.precedence && token.associativity == Associativity::Left
                }) {
                    queue.push_back(stack.pop_front().unwrap())
                }

                stack.push_front(token)
            }

            TokenKind::Parenthesis if token.value == "(" => stack.push_front(token),

            TokenKind::Parenthesis if token.value == ")" => {
                while on_top(&stack, |t| t.value != "(") {
                    queue.push_back(stack.pop_front().unwrap())
                }

                if on_top(&stack, |t| t.value == "(") {
                    stack.pop_front(); // discard left parenthesis
                } else {
                    return Err(anyhow!("Unmatched parenthesis in the token vector"));
                }

                if on_top(&stack, |t| t.kind == TokenKind::Function) {
                    queue.push_back(stack.pop_front().unwrap())
                }
            }

            _ => return Err(anyhow!("Unknown token: {:?}", token)),
        }
    }

    while !stack.is_empty() {
        if on_top(&stack, |t| t.value == "(") {
            return Err(anyhow!("Unmatched parenthesis in the token vector"));
        }

        queue.push_back(stack.pop_front().unwrap())
    }

    return Ok(queue);
}
