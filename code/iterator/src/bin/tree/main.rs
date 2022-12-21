#![recursion_limit="1000"] 
use std::fmt::Display;

enum Node<T> {
    Leaf(T),
    Children(Vec<Node<T>>),
}

impl<T: Display> Node<T> {
    fn traverse(&self, f: &impl Fn(&T)) {
        match self {
            Node::Leaf(x) => f(x),
            Node::Children(children) => {
                for n in children {
                    n.traverse(f);
                }
            }
        }
    }
}

fn main() {
    let tree = Node::Children(vec![
        Node::Leaf(1),
        Node::Children(vec![Node::Leaf(2), Node::Leaf(3), Node::Leaf(4)]),
    ]);
    tree.traverse(&|x| println!("{x}"))
}
