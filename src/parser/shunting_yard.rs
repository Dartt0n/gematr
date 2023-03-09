use super::lexer::{Associativity, Token, TokenValue, UNARY_OPERATOR_PRECEDENCE};
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
        match token.value {
            TokenValue::Literal(_) => queue.push_back(token),

            TokenValue::Function(_) => {
                queue.push_back(Token::delimiter(TokenValue::FunctionArgumentEnd, 0, 0));
                stack.push_front(token);
            }

            TokenValue::CommaSeparator | TokenValue::SemicolonSeparator => {
                while on_top(&stack, |t| {
                    t.value != TokenValue::LeftParenthesis && t.precedence >= token.precedence
                }) {
                    queue.push_back(stack.pop_front().unwrap())
                }
            }

            TokenValue::BinaryMinus
            | TokenValue::BinaryPlus
            | TokenValue::ScalarMultiplication
            | TokenValue::ScalarDivision
            | TokenValue::PowerOperator => {
                let mut token = token;

                if queue
                    .back()
                    .map_or(true, |t| !matches!(t.value, TokenValue::Literal(_)))
                {
                    let new_value = match token.value {
                        TokenValue::BinaryMinus => TokenValue::UnaryMinus,
                        TokenValue::BinaryPlus => TokenValue::UnaryPlus,
                        _ => {
                            return Err(anyhow!(
                                "invalid infix operator {:?} on line {} column {}. asserted it is unary because queue contains {:?}",
                                token.value,
                                token.line,
                                token.column,
                                &queue.back().map(|t| &t.value),
                            ))
                        }
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

            TokenValue::LeftParenthesis => stack.push_front(token),

            TokenValue::RightParenthesis => {
                while on_top(&stack, |t| t.value != TokenValue::LeftParenthesis) {
                    queue.push_back(stack.pop_front().unwrap())
                }

                if on_top(&stack, |t| t.value == TokenValue::LeftParenthesis) {
                    stack.pop_front(); // discard left parenthesis
                } else {
                    return Err(anyhow!("unmatched parenthesis in the token vector"));
                }

                if on_top(&stack, |t| matches!(t.value, TokenValue::Function(_))) {
                    queue.push_back(stack.pop_front().unwrap())
                }
            }

            _ => {
                return Err(anyhow!(
                    "unexpected token {:?} on line {} column {}",
                    &token.value,
                    token.line,
                    token.column
                ))
            }
        }
    }

    while !stack.is_empty() {
        if on_top(&stack, |t| t.value == TokenValue::LeftParenthesis) {
            return Err(anyhow!("unmatched parenthesis in the token vector"));
        }

        queue.push_back(stack.pop_front().unwrap())
    }

    return Ok(queue);
}
