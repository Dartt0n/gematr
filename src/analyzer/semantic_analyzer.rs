use std::rc::Weak;
use std::{collections::VecDeque, rc::Rc};

use super::syntax_tree::{SyntaxNode, SyntaxTree};
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
    queue.push_front(Rc::downgrade(&syntax_tree.get_root().unwrap()));

    while !queue.is_empty() {
        let node = if let Some(node) = queue.pop_back().unwrap().upgrade() {
            node
        } else {
            continue;
        };

        match node.value.kind {
            token::Kind::UnaryOperator(token::UnOps::Plus) => {
                add_children_to_queue(&mut queue, Rc::clone(&node));

                let child = Rc::clone(node.get_children().get(0).unwrap());

                if let Some(parent) = node.get_parent() {
                    SyntaxTree::replace_child(parent, Rc::clone(&node), Rc::clone(&child));
                } else {
                    syntax_tree.set_root(Some(Rc::clone(&child)));
                    child.nullify_parent();
                }

                node.remove_child(child);
            }

            token::Kind::UnaryOperator(token::UnOps::Minus) => {
                let child = Rc::clone(node.get_children().get(0).unwrap());

                if matches!(child.value.kind, token::Kind::UnaryOperator(token::UnOps::Minus)) {
                    let grandchild = Rc::clone(child.get_children().get(0).unwrap());

                    if let Some(parent) = node.get_parent() {
                        SyntaxTree::replace_child(Rc::clone(&parent), Rc::clone(&node), Rc::clone(&grandchild));
                    } else {
                        syntax_tree.set_root(Some(Rc::clone(&grandchild)));
                        grandchild.nullify_parent();
                    }

                    node.remove_child(Rc::clone(&child));
                    child.remove_child(Rc::clone(&grandchild));

                    queue.push_front(Rc::downgrade(&grandchild));
                    add_children_to_queue(&mut queue, grandchild)
                } else if let token::Kind::Number(number) = &child.value.kind {
                    let mut new_token = child.value.clone();
                    new_token.kind = token::Kind::Number("-".to_string() + number);
                    let new_child = Rc::new(SyntaxNode::new(new_token));

                    if let Some(parent) = node.get_parent() {
                        SyntaxTree::replace_child(parent, Rc::clone(&node), new_child);

                        node.remove_child(Rc::clone(&child));
                        child.nullify_parent();
                    } else {
                        syntax_tree.set_root(Some(Rc::clone(&new_child)));
                        new_child.nullify_parent();
                    }
                }
            }

            _ => add_children_to_queue(&mut queue, node),
        }
    }
    Ok(syntax_tree)
}

fn add_children_to_queue(queue: &mut VecDeque<Weak<SyntaxNode>>, node: Rc<SyntaxNode>) {
    for next_node in node.get_children().iter() {
        queue.push_front(Rc::downgrade(next_node));
    }
}
