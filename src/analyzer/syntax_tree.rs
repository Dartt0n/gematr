use std::{collections::VecDeque, fmt::Display};

use super::token::Token;
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct SyntaxTreeNode {
    pub value:    Token,
    pub index:    usize,
    pub parent:   Option<usize>,
    pub children: VecDeque<usize>,
}

#[derive(Debug)]
pub struct ArenaSyntaxTree {
    pub trees: Vec<SyntaxTreeNode>,
}

impl ArenaSyntaxTree {
    pub fn new(root: Token) -> Self {
        Self {
            trees: vec![SyntaxTreeNode {
                value:    root,
                index:    0,
                parent:   None,
                children: VecDeque::new(),
            }],
        }
    }

    pub fn get(&self, index: usize) -> Option<&SyntaxTreeNode> {
        self.trees.get(index)
    }

    pub fn insert(&mut self, parent: usize, child: Token) -> usize {
        let child_id = self.trees.len();
        self.trees[parent].children.push_front(child_id);
        self.trees.push(SyntaxTreeNode {
            value:    child,
            index:    child_id,
            parent:   Some(parent),
            children: VecDeque::new(),
        });
        return child_id;
    }

    fn print_recursive(&self, f: &mut std::fmt::Formatter<'_>, current: usize, indent: usize) -> std::fmt::Result {
        writeln!(f, "{}- {:?}", " ".repeat(indent), self.trees[current].value.kind)?;
        for i in 0..self.trees[current].children.len() {
            self.print_recursive(f, self.trees[current].children[i], indent + 4)?;
        }

        Ok(())
    }
}

impl Display for ArenaSyntaxTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_recursive(f, 0, 0)
    }
}
