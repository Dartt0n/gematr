use std::{
    cell::{Ref, RefCell, RefMut},
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

    pub fn get_parent(&self) -> Option<Rc<SyntaxNode>> {
        self.parent.borrow().upgrade()
    }

    pub fn get_parent_clone(&self) -> Option<Rc<SyntaxNode>> {
        self.get_parent().map(|p| Rc::clone(&p))
    }

    pub fn set_parent(&self, parent: Rc<SyntaxNode>) {
        *self.parent.borrow_mut() = Rc::downgrade(&parent);
    }

    pub fn nullify_parent(&self) {
        *self.parent.borrow_mut() = Weak::new();
    }

    pub fn get_children(&self) -> Ref<Vec<Rc<SyntaxNode>>> {
        self.children.borrow()
    }

    pub fn get_children_mut(&self) -> RefMut<Vec<Rc<SyntaxNode>>> {
        self.children.borrow_mut()
    }

    pub fn add_child(&self, child: Rc<SyntaxNode>) {
        self.get_children_mut().push(child);
    }

    pub fn find_child(&self, child: Rc<SyntaxNode>) -> Option<usize> {
        let child_ptr = Rc::as_ptr(&child);
        let mut index = None;

        for (i, c) in self.get_children().iter().enumerate() {
            let c_ptr = Rc::as_ptr(c);

            if child_ptr == c_ptr {
                index = Some(i);
                break;
            }
        }

        return index;
    }

    pub fn remove_child(&self, child: Rc<SyntaxNode>) {
        if let Some(index) = self.find_child(child) {
            self.get_children_mut().remove(index);
        }
    }
}

impl SyntaxTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn with_root(root_node: Option<Rc<SyntaxNode>>) -> Self {
        Self { root: root_node }
    }

    pub fn set_root(&mut self, node: Option<Rc<SyntaxNode>>) {
        self.root = node;
    }

    pub fn get_root(&self) -> Option<Rc<SyntaxNode>> {
        if let Some(ref r) = self.root {
            Some(Rc::clone(&r))
        } else {
            None
        }
    }

    pub fn add_child(parent: Rc<SyntaxNode>, child: Rc<SyntaxNode>) {
        parent.add_child(child.clone());
        child.set_parent(parent);
    }

    pub fn replace_child(parent: Rc<SyntaxNode>, original_child: Rc<SyntaxNode>, new_child: Rc<SyntaxNode>) {
        if let Some(index) = parent.find_child(original_child) {
            parent.get_children_mut()[index] = Rc::clone(&new_child);
            new_child.set_parent(parent);
        }
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

impl From<SyntaxNode> for Option<Rc<SyntaxNode>> {
    fn from(value: SyntaxNode) -> Self {
        return Some(value.into());
    }
}
