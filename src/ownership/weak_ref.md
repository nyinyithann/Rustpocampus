# Weak Reference
Weak references (also known as weak pointers) are references that do not increase the reference count of an object. They allow objects to have circular references without causing memory leaks.

Weak references are created using the std::rc::Weak type, which is a wrapper around a non-owning reference to an object that is managed by Rc. Unlike Rc, which increments the reference count of an object when it is created, Weak does not increase the reference count and can therefore be used to break reference cycles.

```rust
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
```
