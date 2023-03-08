#[derive(Debug, PartialEq)]
pub enum Associativity {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Operator,
    Literal,
    Parenthesis,
    Separator,
    Function,
}

#[derive(Debug)]
pub struct Coordinates {
    line: usize,
    column: usize,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub associativity: Associativity,
    pub precedence: usize,
    pub coordinates: Coordinates,
}

pub fn tokenize(expression: String) -> Result<Vec<Token>, ()> {
    if expression.len() == 0 {
        return Ok(vec![]);
    }

    let mut tokens = Vec::new();
    let mut char_sequence = expression.chars();

    let mut current_column = 1;
    let mut current_line = 1;

    let mut current_number = String::new();
    let mut current_number_dot_found = false;

    let mut current_function = String::new();

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
            }

            _ if current_number.len() != 0 => {
                tokens.push(Token {
                    kind: TokenKind::Literal,
                    value: current_number.clone(),
                    associativity: Associativity::Left,
                    precedence: 1,
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
                current_function.push(char);
            }

            _ if char.is_alphanumeric() && current_function.len() > 0 => {
                current_function.push(char);
            }

            _ if current_function.len() != 0 => {
                tokens.push(Token {
                    kind: TokenKind::Function,
                    value: current_function.clone(),
                    associativity: Associativity::Left,
                    precedence: 1,
                    coordinates: Coordinates {
                        line: current_line,
                        column: current_column - current_function.len(),
                    },
                });
                current_function = String::new();
            }

            _ => {}
        }

        // Single Char Token Construction
        match char {
            '\n' => current_line += 1,

            '(' | ')' => {
                tokens.push(Token {
                    kind: TokenKind::Parenthesis,
                    value: char.to_string(),
                    associativity: Associativity::Left,
                    precedence: 0,
                    coordinates: Coordinates {
                        line: current_line,
                        column: current_column,
                    },
                });
            }

            '-' | '+' => tokens.push(Token {
                kind: TokenKind::Operator,
                value: char.to_string(),
                associativity: Associativity::Left,
                precedence: 2,
                coordinates: Coordinates {
                    line: current_line,
                    column: current_column,
                },
            }),

            '*' | '/' => tokens.push(Token {
                kind: TokenKind::Operator,
                value: char.to_string(),
                associativity: Associativity::Left,
                precedence: 3,
                coordinates: Coordinates {
                    line: current_line,
                    column: current_column,
                },
            }),

            '^' => tokens.push(Token {
                kind: TokenKind::Operator,
                value: char.to_string(),
                associativity: Associativity::Right,
                precedence: 4,
                coordinates: Coordinates {
                    line: current_line,
                    column: current_column,
                },
            }),

            ',' => tokens.push(Token {
                kind: TokenKind::Separator,
                value: char.to_string(),
                associativity: Associativity::Left,
                precedence: 5,
                coordinates: Coordinates {
                    line: current_line,
                    column: current_column,
                },
            }),

            _ => {}
        }

        current_column += 1;
        cursor = char_sequence.next();
    }

    if current_number.len() != 0 {
        tokens.push(Token {
            kind: TokenKind::Literal,
            value: current_number.clone(),
            associativity: Associativity::Left,
            precedence: 1,
            coordinates: Coordinates {
                line: current_line,
                column: current_column - current_number.len(),
            },
        })
    }

    if current_function.len() != 0 {
        tokens.push(Token {
            kind: TokenKind::Literal,
            value: current_function.clone(),
            associativity: Associativity::Left,
            precedence: 1,
            coordinates: Coordinates {
                line: current_line,
                column: current_column - current_function.len(),
            },
        })
    }

    return Ok(tokens);
}
