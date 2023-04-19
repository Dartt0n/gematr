use std::{collections::VecDeque, rc::Rc};

use super::syntax_tree::SyntaxTree;
use super::token;
use anyhow::{anyhow, Result};

pub fn process(syntax_tree: &SyntaxTree) -> Result<()> {
    if syntax_tree.get_root().is_none() {
        return Err(anyhow!("empty syntax tree"));
    }

    let mut queue = VecDeque::new();
    queue.push_front(syntax_tree.get_root().unwrap());

    while !queue.is_empty() {
        let node = queue.pop_back().unwrap();

        match node.value.kind {
            token::Kind::BinaryOperator(_) => {
                if node.get_children().len() != 2 {
                    return Err(anyhow!("wrong number of arguments for binary operator"));
                }
            }

            token::Kind::UnaryOperator(_) => {
                if node.get_children().len() != 1 {
                    return Err(anyhow!("wrong number of arguments for unary operator"));
                }
            }

            _ => {}
        }

        for child in node.get_children().iter() {
            queue.push_front(Rc::clone(child));
        }
    }

    Ok(())
}

pub fn opimize(syntax_tree: SyntaxTree) -> Result<SyntaxTree> {
    process(&syntax_tree)?;

    let mut syntax_tree = syntax_tree;

    let mut queue = VecDeque::new();
    queue.push_front(syntax_tree.get_root().unwrap());

    while !queue.is_empty() {
        let node = queue.pop_back().unwrap();

        for child in node.get_children().iter() {
            queue.push_front(Rc::clone(child));
        }

        if matches!(node.value.kind, token::Kind::UnaryOperator(token::UnOps::Plus)) {
            let child = Rc::clone(node.get_children().get(0).unwrap());

            if let Some(parent) = node.get_parent() {
                SyntaxTree::replace_child(parent, Rc::clone(&node), Rc::clone(&child));
            } else {
                syntax_tree.set_root(Some(Rc::clone(&child)));
                child.nullify_parent();
            }

            node.remove_child(child);
        }
    }

    // TODO: put unary minuses inside numbers
    // TODO: eliminate double unary minuses

    Ok(syntax_tree)
}
