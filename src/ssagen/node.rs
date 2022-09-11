use super::llir::LLIR;
use std::cell::RefCell;
use std::rc::Rc;
use std::vec::Vec;

#[derive(Clone)]
pub struct Node(Rc<RefCell<Node0>>);

pub struct Node0 {
    pub insts: Vec<LLIR>,
    pub children: Vec<Node>,
    pub parents: Vec<Node>,
}

impl Node {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(Node0 {
            insts: Vec::new(),
            predecessor: Vec::new(),
            successor: Vec::new(),
        })))
    }

    pub fn add_child(&self, n: &Node) {
        self.0.borrow_mut().children.push(n);
    }
    
    pub fn add_parent(&self, n: &Node) {
        self.0.borrow_mut().parents.push(n);
    }
}
