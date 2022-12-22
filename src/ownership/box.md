# Box

- When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size (recursive type)
- When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
- When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type (trait object)

```rust
// when b goes out of scope, the deallocation happens both for the box(sotred on the stack)
// and the data it points to (stored on the heap)
fn main() {
    let b = Box::new(5);
    println!("{b}");
}
```

#### Box usage in recursive type
```rust 
#[derive(Debug)]
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>)
}

fn main() {
    use crate::List::{Nil, Cons};
    let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    dbg!(l); 
}
```

> The Box<T> type is a smart pointer because it implements the [Deref](https://doc.rust-lang.org/std/ops/trait.Deref.html) trait, which allows Box<T> values to be treated like references. When a Box<T> value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the [Drop](https://doc.rust-lang.org/std/mem/fn.drop.html) trait implementation. 
