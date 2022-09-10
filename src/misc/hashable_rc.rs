use std::rc::Rc;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::cmp::PartialEq;

#[derive(Clone)]
pub struct HashableRc<T>(Rc<RefCell<T>>);

impl<T> HashableRc<T> {
    pub fn new(e: Rc<RefCell<T>>) -> Self {
        Self(e)
    }

    fn get_addr(&self) -> usize {
        self.0.as_ptr() as usize
    }
}

impl<T> Hash for HashableRc<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let addr = self.get_addr();
        addr.hash(state);
    }
}

impl<T> PartialEq for HashableRc<T> {
    fn eq(&self, other: &Self) -> bool {
        let self_addr = self.get_addr();
        let oth_addr = other.get_addr();
        self_addr == oth_addr
    } 
}

impl<T> Eq for HashableRc<T> {}
