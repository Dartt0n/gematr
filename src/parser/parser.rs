use super::token::{self, Token, Value};
use anyhow::{anyhow, Result};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(Debug)]
pub struct SyntaxTree {
    pub children: Vec<Rc<RefCell<SyntaxTree>>>,
    pub value:    Token,
    pub parent:   Option<Rc<RefCell<SyntaxTree>>>,
}

pub fn parse(expression: Vec<Token>) {
    println!("RPN:");
    for token in rpn(expression).unwrap() {
        print!("{:?} ", token.value);
    }

    // TODO: build syntax tree & optimize it
    // TODO: error reporting
}

pub fn rpn(tokens: Vec<Token>) -> Result<VecDeque<Token>> {
    let mut stack = VecDeque::<Token>::new();
    let mut queue = VecDeque::<Token>::with_capacity(tokens.len());

    for token in tokens {
        match token.value {
            Value::BinaryPlus
            | Value::UnaryPlus
            | Value::BinaryMinus
            | Value::UnaryMinus
            | Value::ScalarMultiplication
            | Value::ScalarDivision
            | Value::PowerOperator => put_operator(&mut stack, &mut queue, token),

            Value::Literal(_) => queue.push_back(token),
            Value::Function(_) => put_function(&mut stack, &mut queue, token),
            Value::CommaSeparator => put_separator(&mut stack, &mut queue, token),

            Value::LeftParenthesis => stack.push_front(token),
            Value::RightParenthesis => put_right_parenthesis(&mut stack, &mut queue)?,

            _ => {}
        }
    }

    while !stack.is_empty() {
        queue.push_back(stack.pop_front().unwrap())
    }

    return Ok(queue);
}

fn put_operator(stack: &mut VecDeque<Token>, queue: &mut VecDeque<Token>, op_token: Token) {
    while stack.front().map_or(false, |t| {
        t.precedence > op_token.precedence
            || t.precedence == op_token.precedence && op_token.associativity == token::Associativity::Left
    }) {
        queue.push_back(stack.pop_front().unwrap())
    }

    stack.push_front(op_token);
}

fn put_separator(stack: &mut VecDeque<Token>, queue: &mut VecDeque<Token>, sep_token: Token) {
    while stack.front().map_or(false, |t| {
        t.value != Value::LeftParenthesis && t.precedence >= sep_token.precedence
    }) {
        queue.push_back(stack.pop_front().unwrap())
    }
}

fn put_function(stack: &mut VecDeque<Token>, queue: &mut VecDeque<Token>, fn_token: Token) {
    queue.push_back(Token::delimiter(Value::FunctionArgumentEnd, 0, 0));
    stack.push_front(fn_token);
}

fn put_right_parenthesis(stack: &mut VecDeque<Token>, queue: &mut VecDeque<Token>) -> Result<()> {
    while stack.front().map_or(false, |t| t.value != Value::LeftParenthesis) {
        queue.push_back(stack.pop_front().unwrap())
    }

    if stack.front().map_or(false, |t| t.value == Value::LeftParenthesis) {
        stack.pop_front(); // discard left parenthesis
    } else {
        return Err(anyhow!("unmatched parenthesis in the token vector"));
    }

    if stack.front().map_or(false, |t| matches!(t.value, Value::Function(_))) {
        queue.push_back(stack.pop_front().unwrap())
    }

    Ok(())
}
