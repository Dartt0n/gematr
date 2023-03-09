use std::cell::{Ref, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;
use crate::parser::lexer::Token;
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct SyntaxTree {
    children: Vec<Rc<RefCell<SyntaxTree>>>,
    value: Token,
    parent: Option<Rc<RefCell<SyntaxTree>>>
}


impl SyntaxTree{
    pub fn build(rpn_tokens: VecDeque<Token>) -> Result<SyntaxTree> {


        return Err(anyhow!("Unimplemented"))
    }
}

