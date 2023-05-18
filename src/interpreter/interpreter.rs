use std::{collections::{VecDeque, HashMap}, rc::Rc, cell::RefCell};
use crate::{analyzer::syntax_tree::{SyntaxTree, SyntaxNode}, interpreter::engine};
use rust_decimal::Decimal;


pub fn interprete(tree: SyntaxTree) -> Rc<Decimal> {
    let stack = RefCell::new(VecDeque::new());
    
    tree.bfs( |node| {
        stack.borrow_mut().push_back(node);
    });

    let mut enviroment = HashMap::<*const SyntaxNode, Rc<Decimal>>::new();

    while !stack.borrow().is_empty() {
        let node = stack.borrow_mut().pop_back().unwrap();
        
        let mut arguments = Vec::new();

        for child in node.get_children().iter() {
            arguments.push(Rc::clone(&enviroment.get(&Rc::as_ptr(child)).unwrap()));
        }

        let value = engine::evaluate(node.value.kind.clone(), arguments);

        enviroment.insert(Rc::as_ptr(&node), value);
    }

    return Rc::clone(enviroment.get(&Rc::as_ptr(&tree.get_root().unwrap())).unwrap());
}