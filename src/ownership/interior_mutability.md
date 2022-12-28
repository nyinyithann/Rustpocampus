# Interior Mutability

- Mutating the value inside immutable value is called interior mutability
- With [`RefCell<T>`](https://doc.rust-lang.org/std/cell/struct.RefCell.html), the following borrowing rules are enfored at __runtime__. If the rules are broken, the program will panic and exit
    - At any given time, you can have either (but not both) one mutable reference or any number of immutable references.
    - References must always be valid.
- `Cell<T>` and `RefCell<T>` are not thread safe. Use `Mutex<T>`, `RwLock<T>` or `atomic` types for multi threading.

#### Example [`Cell<T>`](https://doc.rust-lang.org/std/cell/struct.Cell.html)
```rust 
use std::cell::Cell;

struct Counter {
    value : Cell<i32>
}

fn main() {
    let c = Counter { value : Cell::new(0) };
    println!("{:?}", c.value);
    c.value.set(1);
    println!("{:?}", c.value);
}
```

#### Basic Example [`RefCell<T>`](https://doc.rust-lang.org/std/cell/struct.RefCell.html)
```rust 
use std::cell::RefCell;

struct Foo {
   value : RefCell<i32>,
}

fn main() {
    let foo = Foo {
        value : RefCell::new(5)
    };
    println!("{:?}", foo.value);

    *foo.value.borrow_mut() = 10;
    println!("{:?}", foo.value);
}
```
#### `RefCell<T>` dynamically enforces the borrow checking rules and panic if violates.
```rust
fn main() {
    use std::cell::RefCell;
    let a = RefCell::new(String::from("hello"));
    let alias = a.borrow();
    println!("{alias}");

    let mut m = a.borrow_mut(); // panic here cause already borrowed mutably
    m.push_str(" world");
}
```

#### With `Rc<T>` that holds `RefCell<T>`, a value can have mutiple owners and can be mutated.
```rust 
use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{ Nil, Cons };

#[derive(Debug)]
enum List<T> {
    Nil,
    Cons(Rc<RefCell<T>>, Rc<List<T>>),
}

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() = 10;

    println!("{a:?}");
    println!("{b:?}");
    println!("{c:?}");
}
```
