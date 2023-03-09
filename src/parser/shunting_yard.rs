use super::lexer::{Associativity, Token, TokenKind, TokenValue, UNARY_OPERATOR_PRECEDENCE};
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
                queue.push_back(Token::delimiter(TokenValue::FunctionArgumentEnd, 0, 0));
                stack.push_front(token);
            }

            TokenKind::Separator => {
                while on_top(&stack, |t| {
                    t.value != TokenValue::LeftParenthesis && t.precedence >= token.precedence
                }) {
                    queue.push_back(stack.pop_front().unwrap())
                }
            }

            TokenKind::Operator => {
                let mut token = token;
                if queue.back().map_or(true, |t| t.kind != TokenKind::Literal) {
                    // map binary token to unary one
                    let new_value = match token.value {
                        TokenValue::BinaryMinus => TokenValue::UnaryMinus,
                        TokenValue::BinaryPlus => TokenValue::UnaryPlus,
                        TokenValue::PowerOperator => TokenValue::PowerOperator,
                        _ => return Err(anyhow!("invalid infix operator")),
                    };

                    token = Token::operator(
                        new_value,
                        Associativity::Right,
                        UNARY_OPERATOR_PRECEDENCE,
                        token.line,
                        token.column,
                    )
                }

                while on_top(&stack, |t| {
                    t.precedence > token.precedence
                        || t.precedence == token.precedence && token.associativity == Associativity::Left
                }) {
                    queue.push_back(stack.pop_front().unwrap())
                }

                stack.push_front(token)
            }

            TokenKind::Parenthesis if token.value == TokenValue::LeftParenthesis => stack.push_front(token),

            TokenKind::Parenthesis if token.value == TokenValue::RightParenthesis => {
                while on_top(&stack, |t| t.value != TokenValue::LeftParenthesis) {
                    queue.push_back(stack.pop_front().unwrap())
                }

                if on_top(&stack, |t| t.value == TokenValue::LeftParenthesis) {
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
        if on_top(&stack, |t| t.value == TokenValue::LeftParenthesis) {
            return Err(anyhow!("Unmatched parenthesis in the token vector"));
        }

        queue.push_back(stack.pop_front().unwrap())
    }

    return Ok(queue);
}
