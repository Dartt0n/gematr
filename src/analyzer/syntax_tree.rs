use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    rc::{Rc, Weak},
};

use super::token::Token;

#[derive(Debug)]
pub struct SyntaxTree {
    pub root: Option<Rc<SyntaxNode>>,
}

#[derive(Debug)]
pub struct SyntaxNode {
    pub value:    Token,
    pub children: RefCell<Vec<Rc<SyntaxNode>>>,
    pub parent:   RefCell<Weak<SyntaxNode>>,
}

impl SyntaxNode {
    pub fn new(value: Token) -> Self {
        Self {
            value:    value,
            children: RefCell::new(vec![]),
            parent:   RefCell::new(Weak::new()),
        }
    }

    fn print_recursive(&self, ind: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
        let indent = " ".repeat(ind);
        write!(f, "{indent}{:?}\n", self.value.kind)?;
        for child in self.children.borrow().iter() {
            child.print_recursive(ind + 6, f)?;
        }

        Ok(())
    }
}

impl SyntaxTree {
    pub fn new() -> Self {
        Self { root: None }
    }
}

impl Display for SyntaxNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.print_recursive(2, f)
    }
}

impl Display for SyntaxTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(node) = &self.root {
            writeln!(f, "{}", node)
        } else {
            writeln!(f, "{{}}")
        }
    }
}
