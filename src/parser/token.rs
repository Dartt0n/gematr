use rust_decimal::Decimal;

#[repr(usize)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    DEFAULT         = 0,
    LITERAL         = 1,
    OPERATOR_LOW    = 2,
    OPERATOR_MEDIUM = 3,
    OPERATOR_HIGH   = 4,
    FUNCTION        = 5,
    OPERATOR_UNARY  = 6,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Associativity {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
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
    pub value:         Value,
    pub associativity: Associativity,
    pub precedence:    Precedence,
    pub line:          usize,
    pub column:        usize,
}

impl Token {
    pub fn literal(literal: Value, line: usize, column: usize) -> Token {
        Token {
            value:         literal,
            associativity: Associativity::Left,
            precedence:    Precedence::LITERAL,
            line:          line,
            column:        column,
        }
    }

    pub fn function(function: Value, line: usize, column: usize) -> Token {
        Token {
            value:         function,
            associativity: Associativity::Left,
            precedence:    Precedence::FUNCTION,
            line:          line,
            column:        column,
        }
    }

    pub fn separator(separator: Value, line: usize, column: usize) -> Token {
        Token {
            value:         separator,
            associativity: Associativity::Left,
            precedence:    Precedence::DEFAULT,
            line:          line,
            column:        column,
        }
    }

    pub fn operator(operator: Value, assoc: Associativity, prec: Precedence, line: usize, column: usize) -> Token {
        Token {
            value:         operator,
            associativity: assoc,
            precedence:    prec,
            line:          line,
            column:        column,
        }
    }

    pub fn parenthesis(parenthesis: Value, line: usize, column: usize) -> Token {
        Token {
            value:         parenthesis,
            associativity: Associativity::Left,
            precedence:    Precedence::DEFAULT,
            line:          line,
            column:        column,
        }
    }

    pub fn delimiter(delimeter: Value, line: usize, column: usize) -> Token {
        Token {
            value:         delimeter,
            associativity: Associativity::Left,
            precedence:    Precedence::FUNCTION,
            line:          line,
            column:        column,
        }
    }
}
