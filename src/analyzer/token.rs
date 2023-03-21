use anyhow::{anyhow, Result};

#[repr(usize)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    Default        = 0,
    OperatorLow    = 1,
    OperatorMedium = 2,
    OperatorHigh   = 3,
    Function       = 4,
    OperatorUnary  = 5,
}

impl Precedence {
    pub fn for_binary_op(op: &BinOps) -> Self {
        match op {
            BinOps::Plus | BinOps::Minus => Self::OperatorLow,
            BinOps::Mult | BinOps::Div | BinOps::Mod => Self::OperatorMedium,
            BinOps::Pow => Self::OperatorHigh,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Associativity {
    Left,
    Right,
}

impl Associativity {
    pub fn for_binary_op(op: &BinOps) -> Self {
        match op {
            BinOps::Plus | BinOps::Minus | BinOps::Mult | BinOps::Div | BinOps::Mod => Self::Left,
            BinOps::Pow => Self::Right,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BinOps {
    Plus,
    Minus,
    Mult,
    Div,
    Mod,
    Pow,
}

impl TryFrom<char> for BinOps {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Plus),
            '-' => Ok(Self::Minus),
            '*' => Ok(Self::Mult),
            '%' => Ok(Self::Mod),
            '/' => Ok(Self::Div),
            '^' => Ok(Self::Pow),
            _ => Err(anyhow!("failed to convert \"{}\" to binary operator", value)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum UnOps {
    Plus,
    Minus,
}

impl TryFrom<char> for UnOps {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Plus),
            '-' => Ok(Self::Minus),
            _ => Err(anyhow!("failed to convert \"{}\" to unary operator", value)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Paren {
    Close,
    Open,
}

impl TryFrom<char> for Paren {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Self::Open),
            ')' => Ok(Self::Close),
            _ => Err(anyhow!("failed to convert \"{}\" to parenthesis", value)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Delim {
    FuncArgs,
    Comma,
}

impl TryFrom<char> for Delim {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            ',' => Ok(Self::Comma),
            _ => Err(anyhow!("failed to convert \"{}\" to delimiter", value)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Kind {
    Number(String),
    Func(String),
    Parenthesis(Paren),
    BinaryOperator(BinOps),
    UnaryOperator(UnOps),
    Delimeter(Delim),
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub assoc:  Associativity,
    pub prec:   Precedence,
    pub kind:   Kind,
    pub line:   usize,
    pub column: usize,
}

impl Token {
    pub fn number(number: String, line: usize, column: usize) -> Self {
        Self {
            assoc:  Associativity::Left,
            prec:   Precedence::Default,
            kind:   Kind::Number(number),
            line:   line,
            column: column,
        }
    }

    pub fn function(func: String, line: usize, column: usize) -> Self {
        Self {
            assoc:  Associativity::Left,
            prec:   Precedence::Function,
            kind:   Kind::Func(func),
            line:   line,
            column: column,
        }
    }

    pub fn parenthesis(paren: char, line: usize, column: usize) -> Result<Self> {
        Ok(Token {
            assoc: Associativity::Left,
            prec: Precedence::Default,
            kind: Kind::Parenthesis(Paren::try_from(paren)?),
            line,
            column,
        })
    }

    pub fn unary_op(op: char, line: usize, column: usize) -> Result<Self> {
        Ok(Self {
            assoc: Associativity::Right,
            prec: Precedence::OperatorUnary,
            kind: Kind::UnaryOperator(UnOps::try_from(op)?),
            line,
            column,
        })
    }

    pub fn binary_op(op: char, line: usize, column: usize) -> Result<Self> {
        let op = BinOps::try_from(op)?;

        Ok(Self {
            assoc:  Associativity::for_binary_op(&op),
            prec:   Precedence::for_binary_op(&op),
            kind:   Kind::BinaryOperator(op),
            line:   line,
            column: column,
        })
    }

    pub fn delimiter(delim: char, line: usize, column: usize) -> Result<Self> {
        Ok(Self {
            assoc:  Associativity::Left,
            prec:   Precedence::Default,
            kind:   Kind::Delimeter(Delim::try_from(delim)?),
            line:   line,
            column: column,
        })
    }

    pub fn util_delimiter(delim: Delim) -> Self {
        Self {
            assoc:  Associativity::Left,
            prec:   Precedence::Default,
            kind:   Kind::Delimeter(delim),
            line:   0,
            column: 0,
        }
    }
}
