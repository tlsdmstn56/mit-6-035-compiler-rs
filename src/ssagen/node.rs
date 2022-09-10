use std::vec::Vec;
use super::llir::LLIR;
use std::rc::Rc;
use std::cell::RefCell;

pub type Node = Rc<RefCell<Node0>>;

pub struct Node0 {
    pub insts: Vec<LLIR>,
    pub predecessor: Vec<Node>,
    pub successor: Vec<Node>,
}

pub fn create_node() -> Node {
    Rc::new(RefCell::new(Node0{
        insts: Vec::new(),
        predecessor: Vec::new(),
        successor: Vec::new(),
    }))
}


