use anyhow::{anyhow, Result};

use super::token::{self, Token};

pub fn tokenize<CharIter>(char_stream: CharIter) -> Result<Vec<Token>>
where
    CharIter: Iterator<Item = char>,
{
    let mut tokens = Vec::new();

    let mut crnt_number = String::new();
    let mut crnt_num_dot_found = false;

    let mut crnt_func = String::new();

    let mut crnt_line = 0;
    let mut crnt_column = 0;

    for cc in char_stream {
        crnt_column += 1;

        if cc.is_digit(10) {
            crnt_number.push(cc);
            continue;
        }

        if cc == '.' && !crnt_num_dot_found {
            crnt_number.push(cc);
            crnt_num_dot_found = true;
            continue;
        }

        if cc == '.' && crnt_num_dot_found {
            return Err(anyhow!(
                "met unexpected dot on line {} column {}",
                crnt_line,
                crnt_column
            ));
        }

        if cc.is_alphabetic() && crnt_func.is_empty() {
            crnt_func.push(cc);
            continue;
        }

        if cc.is_alphanumeric() && !crnt_func.is_empty() {
            crnt_func.push(cc);
            continue;
        }

        if !crnt_number.is_empty() {
            tokens.push(Token::number(
                crnt_number.clone(),
                crnt_line,
                crnt_column - crnt_number.len() + 1,
            ));

            crnt_number = String::new();
            crnt_num_dot_found = false;
        }

        if !crnt_func.is_empty() {
            tokens.push(Token::function(
                crnt_func.clone(),
                crnt_line,
                crnt_column - crnt_func.len() + 1,
            ));

            crnt_func = String::new();
        }

        if cc.is_whitespace() {
            if cc == '\n' {
                crnt_line += 1;
            }
            continue;
        }

        if let Ok(t) = Token::parenthesis(cc, crnt_line, crnt_column) {
            if tokens.last().map_or(false, |t| matches!(t.kind, token::Kind::Func(_)))
                && t.kind == token::Kind::Parenthesis(token::Paren::Open)
            {
                tokens.push(t);
                tokens.push(Token::util_delimiter(token::Delim::FuncArgs));
            } else {
                tokens.push(t);
            }

            continue;
        }

        if let Ok(t) = Token::delimiter(cc, crnt_line, crnt_column) {
            tokens.push(t);
            continue;
        }

        if tokens.last().map_or(true, |t| {
            matches!(t.kind, token::Kind::Parenthesis(token::Paren::Open))
                || matches!(t.kind, token::Kind::BinaryOperator(_))
                || matches!(t.kind, token::Kind::UnaryOperator(_))
        }) {
            tokens.push(Token::unary_op(cc, crnt_line, crnt_column)?);
            continue;
        }

        if let Ok(t) = Token::binary_op(cc, crnt_line, crnt_column) {
            tokens.push(t);
            continue;
        }

        // TODO: report system
        println!(
            "warning: symbol '{}' on line {} column {} is ignored",
            cc, crnt_line, crnt_column
        );

        crnt_column += 1;
    }
    if !crnt_number.is_empty() {
        tokens.push(Token::number(
            crnt_number.clone(),
            crnt_line,
            crnt_column - crnt_number.len() + 1,
        ));
    }

    if !crnt_func.is_empty() {
        tokens.push(Token::function(
            crnt_func.clone(),
            crnt_line,
            crnt_column - crnt_func.len() + 1,
        ));
    }

    Ok(tokens)
}
