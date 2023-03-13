use rust_decimal::Decimal;

#[repr(usize)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    Default        = 0,
    Literal        = 1,
    OperatorLow    = 2,
    OperatorMedium = 3,
    OperatorHigh   = 4,
    Function       = 5,
    OperatorUnary  = 6,
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
            precedence:    Precedence::Literal,
            line:          line,
            column:        column,
        }
    }

    pub fn function(function: Value, line: usize, column: usize) -> Token {
        Token {
            value:         function,
            associativity: Associativity::Left,
            precedence:    Precedence::Function,
            line:          line,
            column:        column,
        }
    }

    pub fn separator(separator: Value, line: usize, column: usize) -> Token {
        Token {
            value:         separator,
            associativity: Associativity::Left,
            precedence:    Precedence::Default,
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
            precedence:    Precedence::Default,
            line:          line,
            column:        column,
        }
    }

    pub fn delimiter(delimeter: Value, line: usize, column: usize) -> Token {
        Token {
            value:         delimeter,
            associativity: Associativity::Left,
            precedence:    Precedence::Function,
            line:          line,
            column:        column,
        }
    }
}
