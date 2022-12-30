# Closure

- Unlike functions, Rust supports type inference for closure - type annotation is optional.
- Closures can capture values from their environment in three ways:
    - borrowing immutably
    - borrowing mutably
    - taking ownership
- Depending on what the body of the function does with the captured values, closure will decide which one of above three to be used.

### Borrowing immutably
```rust
fn main() {
    // Rust allows multiple immutable references
    let list = vec![1, 2, 3];
    // list is immutably borrowed here
    let immutable_borrow = || println!("borrowed list: {:?}", list);
    println!("before immutable borrow: {:?}", list);
    immutable_borrow();
    println!("before immutable borrow: {:?}", list);
    immutable_borrow();
    println!("before immutable borrow: {:?}", list);
    immutable_borrow();
    println!("after all borrow, list is {:?}", list);
}
```
### Borrowing mutably
```rust
fn main() {
    let mut list = vec![1, 2, 3];
    let mut mutable_borrow = || list.push(4);
    mutable_borrow();
    mutable_borrow();
    println!("after mutable borrow, list is {:?}", list);
}
```

### Closure ownership 
Use `move` keyword to force closure to take ownership of its environment values
```rust
use std::thread;

fn main() {
    let mut list = vec![1, 2, 3];
    println!("before mutable borrow: {:?}", list);
   
    thread::spawn(move || println!("From thread: {:?}", list))
    .join()
    .unwrap();
}
```
For `copy` type, closure will copy it.
```rust
fn main() {
    let a = 10;
    let move_closure = move |x : i32| a + x;
    println!("{}", move_closure(2));
    println!("a is still here {a}");
}
```
Sometimes, a closure must take ownership of an environment variable to be valid and it happens automatically without `move`.

```rust
// magic_string is immutably borrowed here
fn main() {
    let magic_string = String::from("abracadaba");
    let closure = || println!("{:?}", magic_string);
    closure();
    println!("{:?}", magic_string);
}
```

```rust
// magic_string is moved here and 
// closure takes ownership without needing to use 'move' keyword
fn main() {
    let magic_string = String::from("abracadaba");
    let _closure = || magic_string;
    println!("{:?}", magic_string); // throw error here
}
```
> A closure that take ownership of environment variable(s) can be called only **once**.
