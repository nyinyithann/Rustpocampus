enum Node<T> {
    Leaf(T),
    Children(Vec<Node<T>>)
}

impl<T> Node<T> {
    fn traverse (&self, f : impl Fn(&T)) {
        match 
    }
}

fn main() {
    println!("hello");
}
