use std::{rc::{Rc, Weak}, cell::RefCell, borrow::Borrow};

#[derive(Debug)]
struct Node {
    value : i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let a = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });

    let b = Rc::new(Node{
        value: 10,
        parent: RefCell::new(Rc::downgrade(&a)),
        children: RefCell::new(vec![])
    });

    a.children.borrow_mut().push(Rc::clone(&b));
    dbg!(a);
}
