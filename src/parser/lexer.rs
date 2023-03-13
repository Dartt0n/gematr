use std::str::FromStr;

use super::token::{self, Token};
use anyhow::{anyhow, Context, Result};
use rust_decimal::Decimal;

pub fn tokenize(expression: &str) -> Result<Vec<Token>> {
    if expression.len() == 0 {
        return Ok(vec![]);
    }

    let mut tokens = Vec::new();

    let mut current_number = String::new();
    let mut current_number_dot_found = false;

    let mut current_function = String::new();

    let mut char_sequence = expression.chars();
    let mut cursor;
    let mut current_column = 0;
    let mut current_line = 1;

    while {
        cursor = char_sequence.next();
        current_column += 1;
        cursor.is_some()
    } {
        let char = cursor.unwrap();

        // Skip whitespaces
        if char.is_whitespace() {
            if char == '\n' {
                current_line += 1;
            }
            continue;
        }

        // Build number (as string) iteratively
        if char.is_digit(10) {
            current_number.push(char);
            continue;
        }
        if char == '.' && !current_number_dot_found {
            current_number.push(char);
            current_number_dot_found = true;
            continue;
        }
        if char == '.' && current_number_dot_found {
            return Err(anyhow!(
                "unexpected dot met on line {} column {}:\n\t{}\n\t{}",
                current_line,
                current_column,
                &expression,
                " ".repeat(current_column - 1) + "^",
            ));
        }

        if current_number.len() != 0 {
            let value = Decimal::from_str(&current_number)
                .with_context(|| format!("failed to convert \"{}\" to number", &current_number))?;

            tokens.push(Token::literal(
                token::Value::Literal(value),
                current_line,
                current_column - current_number.len() + 1,
            ));

            current_number_dot_found = false;
            current_number = String::new();
        }

        // Build function name iteratively
        if char.is_alphabetic() {
            current_function.push(char);
            continue;
        }

        if char.is_alphanumeric() && current_function.len() > 0 {
            current_function.push(char);
            continue;
        }

        if current_function.len() != 0 {
            tokens.push(Token::function(
                token::Value::Function(current_function.clone()),
                current_line,
                current_column,
            ));

            current_function = String::new();
        }

        match char {
            '(' => tokens.push(Token::parenthesis(
                token::Value::LeftParenthesis,
                current_line,
                current_column,
            )),
            ')' => tokens.push(Token::parenthesis(
                token::Value::RightParenthesis,
                current_line,
                current_column,
            )),
            '-' => tokens.push(Token::operator(
                token::Value::BinaryMinus,
                token::Associativity::Left,
                token::Precedence::OperatorLow,
                current_line,
                current_column,
            )),
            '+' => tokens.push(Token::operator(
                token::Value::BinaryPlus,
                token::Associativity::Left,
                token::Precedence::OperatorLow,
                current_line,
                current_column,
            )),
            '*' => tokens.push(Token::operator(
                token::Value::ScalarMultiplication,
                token::Associativity::Left,
                token::Precedence::OperatorMedium,
                current_line,
                current_column,
            )),
            '/' => tokens.push(Token::operator(
                token::Value::ScalarDivision,
                token::Associativity::Left,
                token::Precedence::OperatorMedium,
                current_line,
                current_column,
            )),
            '^' => tokens.push(Token::operator(
                token::Value::PowerOperator,
                token::Associativity::Right,
                token::Precedence::OperatorHigh,
                current_line,
                current_column,
            )),
            ',' => tokens.push(Token::separator(
                token::Value::CommaSeparator,
                current_line,
                current_column,
            )),

            unexpected_char => {
                return Err(anyhow!(
                    "met unexpected character \'{}\' on line {} column {}:\n\t{}\n\t{}",
                    unexpected_char,
                    current_line,
                    current_column,
                    &expression,
                    " ".repeat(current_column - 1) + "^",
                ))
            }
        }
    }

    if current_number.len() != 0 {
        let value = Decimal::from_str(&current_number)
            .with_context(|| format!("failed to convert \"{}\" to number", &current_number))?;

        tokens.push(Token::literal(
            token::Value::Literal(value),
            current_line,
            current_column - current_number.len() + 1,
        ));
    }

    // Error case, because each function must be followed by parentheses
    if current_function.len() != 0 {
        tokens.push(Token::function(
            token::Value::Function(current_function.clone()),
            current_line,
            current_column - current_function.len() + 1,
        ))
    }

    // todo: refactor!!!

    if (tokens[0].value == token::Value::BinaryMinus || tokens[0].value == token::Value::BinaryPlus)
        && (matches!(tokens[1].value, token::Value::Literal(_))
            || matches!(tokens[1].value, token::Value::Function(_))
            || matches!(tokens[1].value, token::Value::LeftParenthesis))
    {
        let value = if tokens[0].value == token::Value::BinaryPlus {
            token::Value::UnaryPlus
        } else {
            token::Value::UnaryMinus
        };

        tokens[0] = Token::operator(
            value,
            token::Associativity::Left,
            token::Precedence::OperatorUnary,
            tokens[0].line,
            tokens[0].column,
        )
    }

    for i in 1..tokens.len() - 1 {
        if (tokens[i - 1].value == token::Value::BinaryPlus
            || tokens[i - 1].value == token::Value::BinaryMinus
            || tokens[i - 1].value == token::Value::ScalarDivision
            || tokens[i - 1].value == token::Value::ScalarMultiplication
            || tokens[i - 1].value == token::Value::PowerOperator
            || tokens[i - 1].value == token::Value::LeftParenthesis)
            && (tokens[i].value == token::Value::BinaryMinus || tokens[i].value == token::Value::BinaryPlus)
            && (matches!(tokens[i + 1].value, token::Value::Literal(_))
                || matches!(tokens[i + 1].value, token::Value::Function(_))
                || matches!(tokens[i + 1].value, token::Value::LeftParenthesis))
        {
            let value = if tokens[i].value == token::Value::BinaryPlus {
                token::Value::UnaryPlus
            } else {
                token::Value::UnaryMinus
            };

            tokens[i] = Token::operator(
                value,
                token::Associativity::Left,
                token::Precedence::OperatorUnary,
                tokens[i].line,
                tokens[i].column,
            )
        }
    }

    return Ok(tokens);
}
