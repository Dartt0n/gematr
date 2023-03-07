#[derive(Debug)]
pub enum TokenKind {
    Operator,
    Literal,
    Parenthesis,
    Separator,
    Symbol,
}

#[derive(Debug)]
pub struct Coordinates {
    line: usize,
    column: usize,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    value: String,
    coordinates: Coordinates,
}

pub fn tokenize(expression: String) -> Result<Vec<Token>, ()> {
    if expression.len() == 0 {
        return Ok(vec!());
    }

    let mut tokens = Vec::new();
    let mut char_sequence = expression.chars();

    let mut current_column = 1;
    let mut current_line = 1;

    let mut current_number = String::new();
    let mut current_number_dot_found = false;

    let mut current_symbol = String::new();

    let mut cursor = char_sequence.next();
    while cursor.is_some() {
        let char = cursor.unwrap();

        // Sequential Token Construction: Number
        match char {
            _ if char.is_digit(10) => {
                current_number.push(char);
            }

            '.' if !current_number_dot_found => {
                current_number.push(char);
                current_number_dot_found = true;
            }

            '.' if current_number_dot_found => {
                return Err(());
            },

            _ if current_number.len() != 0 => {
                tokens.push(Token {
                    kind: TokenKind::Literal,
                    value: current_number.clone(),
                    coordinates: Coordinates {
                        line: current_line,
                        column: current_column - current_number.len(),
                    },
                });
                current_number_dot_found = false;
                current_number = String::new();
            }

            _ => {}
        }

        // Sequential Token Construction: Variable or Function
        match char {
            _ if char.is_alphabetic() => {
                current_symbol.push(char);
            }

            _ if char.is_alphanumeric() && current_symbol.len() > 0 => {
                current_symbol.push(char);
            }

            _ if current_symbol.len() != 0 => {
                tokens.push(Token {
                    kind: TokenKind::Symbol,
                    value: current_symbol.clone(),
                    coordinates: Coordinates {
                        line: current_line,
                        column: current_column - current_symbol.len(),
                    },
                });
                current_symbol = String::new();
            }

            _ => {}
        }

        // Single Char Token Construction
        match char {
            '\n' => { current_line += 1 }

            '(' | ')' => {
                tokens.push(Token {
                    kind: TokenKind::Parenthesis,
                    value: char.to_string(),
                    coordinates: Coordinates { line: current_line, column: current_column },
                });
            }

            '-' | '+' | '*' | '/' | '^' => {
                tokens.push(Token {
                    kind: TokenKind::Operator,
                    value: char.to_string(),
                    coordinates: Coordinates { line: current_line, column: current_column },
                })
            }

            ',' => {
                tokens.push(Token {
                    kind: TokenKind::Separator,
                    value: char.to_string(),
                    coordinates: Coordinates { line: current_line, column: current_column },
                })
            }

            _ => {}
        }

        current_column += 1;
        cursor = char_sequence.next();
    }

    return Ok(tokens);
}