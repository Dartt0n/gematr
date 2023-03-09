use anyhow::{anyhow, Context, Result};
use rust_decimal::Decimal;
use std::str::FromStr;

pub const DEFAULT_PRECEDENCE: usize = 0;
pub const LITERAL_PRECEDENCE: usize = 1;
pub const OPERATOR_LOW_PRECEDENCE: usize = 2;
pub const OPERATOR_MEDIUM_PRECEDENCE: usize = 3;
pub const OPERATOR_HIGH_PRECEDENCE: usize = 4;
pub const FUNCTION_PRECEDENCE: usize = 5;
pub const UNARY_OPERATOR_PRECEDENCE: usize = 6;

#[derive(Debug, PartialEq, Clone)]
pub enum Associativity {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Operator,
    Literal,
    Parenthesis,
    Separator,
    Function,
    Delimiter, // used to separate data, e.g. function arguments
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    BinaryPlus,
    UnaryPlus,
    BinaryMinus,
    UnaryMinus,
    ScalarMultiplication,
    ScalarDivision,
    PowerOperator,

    Literal(Decimal),

    LeftParenthesis,
    RightParenthesis,

    CommaSeparator,
    SemicolonSeparator,

    Function(String),

    FunctionArgumentEnd,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: TokenValue,
    pub associativity: Associativity,
    pub precedence: usize,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn literal(literal: TokenValue, line: usize, column: usize) -> Token {
        Token {
            kind: TokenKind::Literal,
            value: literal,
            associativity: Associativity::Left,
            precedence: LITERAL_PRECEDENCE,
            line,
            column,
        }
    }

    pub fn function(function: TokenValue, line: usize, column: usize) -> Token {
        Token {
            kind: TokenKind::Function,
            value: function,
            associativity: Associativity::Left,
            precedence: FUNCTION_PRECEDENCE,
            line,
            column,
        }
    }

    pub fn separator(separator: TokenValue, line: usize, column: usize) -> Token {
        Token {
            kind: TokenKind::Separator,
            value: separator,
            associativity: Associativity::Left,
            precedence: DEFAULT_PRECEDENCE,
            line,
            column,
        }
    }

    pub fn operator(operator: TokenValue, assoc: Associativity, prec: usize, line: usize, column: usize) -> Token {
        Token {
            kind: TokenKind::Operator,
            value: operator,
            associativity: assoc,
            precedence: prec,
            line,
            column,
        }
    }

    pub fn parenthesis(parenthesis: TokenValue, line: usize, column: usize) -> Token {
        Token {
            kind: TokenKind::Parenthesis,
            value: parenthesis,
            associativity: Associativity::Left,
            precedence: DEFAULT_PRECEDENCE,
            line,
            column,
        }
    }

    pub fn delimiter(delimiter: TokenValue, line: usize, column: usize) -> Token {
        Token {
            kind: TokenKind::Delimiter,
            value: delimiter,
            associativity: Associativity::Left,
            precedence: FUNCTION_PRECEDENCE,
            line,
            column,
        }
    }
}

pub fn tokenize(expression: String) -> Result<Vec<Token>> {
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

        // Sequential Token Construction: Number
        match char {
            _ if char.is_digit(10) => {
                current_number.push(char);
                continue;
            }

            '.' if !current_number_dot_found => {
                current_number.push(char);
                current_number_dot_found = true;
                continue;
            }

            '.' if current_number_dot_found => {
                return Err(anyhow!(
                    "unexpected dot met on line {} column {}",
                    current_line,
                    current_column
                ));
            }

            _ if current_number.len() != 0 => {
                let value = Decimal::from_str(&current_number.clone()).with_context(|| {
                    format!(
                        "failed to convert number {} ot decimal on line {} column {}",
                        &current_number,
                        current_line,
                        current_column - current_number.len() + 1
                    )
                })?;

                tokens.push(Token::literal(
                    TokenValue::Literal(value),
                    current_line,
                    current_column - current_number.len() + 1,
                ));
                current_number_dot_found = false;
                current_number = String::new();

                continue;
            }

            _ => {}
        }

        // Sequential Token Construction: Variable or Function
        match char {
            _ if char.is_alphabetic() => {
                current_function.push(char);
                continue;
            }

            _ if char.is_alphanumeric() && current_function.len() > 0 => {
                current_function.push(char);
                continue;
            }

            _ if current_function.len() != 0 => {
                tokens.push(Token::function(
                    TokenValue::Function(current_function.clone()),
                    current_line,
                    current_column - current_function.len() + 1,
                ));
                current_function = String::new();
                continue;
            }

            _ => {}
        }

        // Single Char Token Construction
        match char {
            '\n' => current_line += 1,

            whitespace if whitespace.is_whitespace() => {}

            '(' => tokens.push(Token::parenthesis(
                TokenValue::LeftParenthesis,
                current_line,
                current_column,
            )),

            ')' => tokens.push(Token::parenthesis(
                TokenValue::RightParenthesis,
                current_line,
                current_column,
            )),
            // unary operators are parsed later during shunting yard
            // todo: recognize unary/binary operator during tokenization
            '-' => tokens.push(Token::operator(
                TokenValue::BinaryMinus,
                Associativity::Left,
                OPERATOR_LOW_PRECEDENCE,
                current_line,
                current_column,
            )),

            '+' => tokens.push(Token::operator(
                TokenValue::BinaryPlus,
                Associativity::Left,
                OPERATOR_LOW_PRECEDENCE,
                current_line,
                current_column,
            )),

            '*' => tokens.push(Token::operator(
                TokenValue::ScalarMultiplication,
                Associativity::Left,
                OPERATOR_MEDIUM_PRECEDENCE,
                current_line,
                current_column,
            )),

            '/' => tokens.push(Token::operator(
                TokenValue::ScalarDivision,
                Associativity::Left,
                OPERATOR_MEDIUM_PRECEDENCE,
                current_line,
                current_column,
            )),

            '^' => tokens.push(Token::operator(
                TokenValue::PowerOperator,
                Associativity::Right,
                OPERATOR_HIGH_PRECEDENCE,
                current_line,
                current_column,
            )),

            ',' => tokens.push(Token::separator(
                TokenValue::CommaSeparator,
                current_line,
                current_column,
            )),

            unexpected_char => {
                return Err(anyhow!(
                    "met unexpected characted \'{}\' on line {} column {}",
                    unexpected_char,
                    current_line,
                    current_column
                ))
            }
        }
    }

    if current_number.len() != 0 {
        let value = Decimal::from_str(&current_number.clone()).with_context(|| {
            format!(
                "failed to convert number {} ot decimal on line {} column {}",
                &current_number,
                current_line,
                current_column - current_number.len() + 1
            )
        })?;

        tokens.push(Token::literal(
            TokenValue::Literal(value),
            current_line,
            current_column - current_number.len() + 1,
        ));
    }

    if current_function.len() != 0 {
        tokens.push(Token::function(
            TokenValue::Function(current_function.clone()),
            current_line,
            current_column - current_function.len() + 1,
        ))
    }

    return Ok(tokens);
}
