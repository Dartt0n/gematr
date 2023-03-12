use anyhow::{anyhow, Result};

// Expr = "(", Expr, ")"
// |   "+", Expr
// |   "-", Expr
// |   Expr, "+", Expr
// |   Expr, "-", Expr
// |   Expr, "*", Expr
// |   Expr, "/", Expr
// |   Expr, "^", Expr
// |   Func, "(", Args, ")"
// |   {Digit};
//
// Digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";
//
// Letter = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" |
// "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" |
// "Y" | "Z" | "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" |
// "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" |
// "y" | "z" ;
//
// Func = (Letter | "_"), [{Letter | digit | "_"}];
//
// Args = ø | Expr [{",", Expr}], [","];

#[derive(Debug)]
pub struct RawToken {
    pub value:  String,
    pub line:   usize,
    pub column: usize,
}

pub fn tokenize(expression: &str) -> Result<Vec<RawToken>> {
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
            tokens.push(RawToken {
                value:  current_number.clone(),
                line:   current_line,
                column: current_column - current_number.len() + 1,
            });

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
            tokens.push(RawToken {
                value:  current_function.clone(),
                line:   current_line,
                column: current_column - current_function.len() + 1,
            });
            current_function = String::new();
        }

        match char {
            '(' | ')' | '-' | '+' | '*' | '/' | '^' | ',' | ',' => tokens.push(RawToken {
                value:  char.to_string(),
                line:   current_line,
                column: current_column,
            }),

            unexpected_char => {
                return Err(anyhow!(
                    "met unexpected characted \'{}\' on line {} column {}:\n\t{}\n\t{}",
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
        tokens.push(RawToken {
            value:  current_number.clone(),
            line:   current_line,
            column: current_column - current_number.len() + 1,
        });
    }

    // Error case, because each function must be followed by parentheses
    if current_function.len() != 0 {
        tokens.push(RawToken {
            value:  current_function.clone(),
            line:   current_line,
            column: current_column - current_function.len() + 1,
        })
    }

    return Ok(tokens);
}
