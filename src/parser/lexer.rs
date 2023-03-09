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

pub const DEFAULT_PRECEDENCE: usize = 0;
pub const LITERAL_PRECEDENCE: usize = 1;
pub const OPERATOR_LOW_PRECEDENCE: usize = 2;
pub const OPERATOR_MEDIUM_PRECEDENCE: usize = 3;
pub const OPERATOR_HIGH_PRECEDENCE: usize = 4;
pub const FUNCTION_PRECEDENCE: usize = 5;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub associativity: Associativity,
    pub precedence: usize,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn literal(number: String, line: usize, column: usize) -> Token {
        Token {
            kind: TokenKind::Literal,
            value: number,
            associativity: Associativity::Left,
            precedence: LITERAL_PRECEDENCE,
            line,
            column,
        }
    }

    pub fn function(function: String, line: usize, column: usize) -> Token {
        Token {
            kind: TokenKind::Function,
            value: function,
            associativity: Associativity::Left,
            precedence: FUNCTION_PRECEDENCE,
            line,
            column,
        }
    }

    pub fn separator(separator: String, line: usize, column: usize) -> Token {
        Token {
            kind: TokenKind::Separator,
            value: separator,
            associativity: Associativity::Left,
            precedence: DEFAULT_PRECEDENCE,
            line,
            column,
        }
    }

    pub fn operator(operator: String, assoc: Associativity, prec: usize, line: usize, column: usize) -> Token {
        Token {
            kind: TokenKind::Operator,
            value: operator,
            associativity: assoc,
            precedence: prec,
            line,
            column,
        }
    }

    pub fn parenthesis(parenthesis: String, line: usize, column: usize) -> Token {
        Token {
            kind: TokenKind::Parenthesis,
            value: parenthesis,
            associativity: Associativity::Left,
            precedence: DEFAULT_PRECEDENCE,
            line,
            column,
        }
    }

    pub fn delimiter(delimiter: String, line:usize, column: usize) -> Token {
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
                tokens.push(Token::literal(current_number.clone(), current_line, current_column - current_number.len() + 1));
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
                tokens.push(Token::function(current_function.clone(), current_line, current_column - current_function.len() + 1));
                current_function = String::new();
            }

            _ => {}
        }

        // Single Char Token Construction
        match char {
            '\n' => current_line += 1,

            '(' | ')' => {
                tokens.push(Token::parenthesis(char.to_string(), current_line, current_column));
            }

            '-' | '+' => tokens.push(Token::operator(char.to_string(), Associativity::Left, OPERATOR_LOW_PRECEDENCE, current_line, current_column)),

            '*' | '/' => tokens.push(Token::operator(char.to_string(), Associativity::Left, OPERATOR_MEDIUM_PRECEDENCE, current_line, current_column)),

            '^' => tokens.push(Token::operator(char.to_string(), Associativity::Right, OPERATOR_HIGH_PRECEDENCE, current_line, current_column)),

            ',' => tokens.push(Token::separator(char.to_string(), current_line, current_column)),

            _ => {}
        }

        current_column += 1;
        cursor = char_sequence.next();
    }

    if current_number.len() != 0 {
        tokens.push(Token::literal(current_number, current_line, current_column - current_number.len() + 1));
    }

    if current_function.len() != 0 {
        tokens.push(Token::function(current_function, current_line, current_column - current_function.len() + 1))
    }

    return Ok(tokens);
}
