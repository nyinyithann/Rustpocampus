# Ownership & Borrowing

- Each value in Rust has an *owner*.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped. ( call `drop`)
- At any given time, you can have either one mutable reference or any number of immutable references. (one writer or multiple readers)
- References must always be valid. 

Rust won't allow double free or dangling pointer in safe mode.

```rust, editable
fn borrow(b: &String) {
    println!("borrow -> {b}");
}

fn mut_borrow(mb: &mut String) {
    *mb = "updated".to_string();
}

fn move_msg(m: String) {
    println!("{m}");
}

fn main() {
    let mut msg = String::from("hello");
    borrow(&msg);
    println!("{msg}");
    mut_borrow(&mut msg);
    println!("{msg}");
    borrow(&msg);
    println!("{msg}");
    move_msg(msg);

    // error here because msg was moved in above line
    println!("{msg}"); 
}
```

#### keywords 
- borrow, move
- copy, drop
- dangling pointer
- double free

### SUMMARY (from a reddit user)
The borrowing and ownership mechanism can be simplified down to:

- Passing a variable by value will move ownership, dropping the original variable from memory
- Passing a variable by mutable reference will keep the original variable, but allow you to modify the variable.
- You may only borrow a variable mutably once at a time, and you may not immutably borrow while mutably borrowing.
- You may have as many immutable borrows as you want, so long as you aren't modifying that value.
- You may mutably borrow a field in a struct, and then mutably borrow a different field in the same struct simultaneously, so long as you aren't also mutably borrowing the overall struct.
- You can use `Cell` and `RefCell` to allow for mutably modifying an immutable field in a struct.
- You may mutably borrow multiple slices from the same array simultaneously so long as there is no overlap.
- Safe memory practices means that instead of mutably borrowing the same variable in multiple places, you queue the changes to make in a separate location and apply them serially one after another.


## From chatgpt

#### Ownership
- Every value in Rust has a single owner.
- When the owner goes out of scope, the value is automatically dropped (freed from memory).
- You can borrow a value from its owner, allowing you to use the value without taking ownership of it.
- When a value is borrowed, the original owner cannot modify the value until the borrow goes out of scope.

```rust, editable
fn main() {
    let x = 5;  // x is a new i32 value with the value 5
    let y = x;  // y is a new i32 value with the value 5
    // x is no longer needed, so it is dropped
    println!("{}", y);  // prints 5
}
```

#### Borrowing
There are two types of borrowing in Rust: immutable borrowing and mutable borrowing.

Immutable borrowing is done using the & operator. It allows you to use a value without changing it, but it does not allow you to modify the value.


```rust, editable
fn main() {
    let x = 5;
    let y = &x;  // y is an immutable borrow of x
    println!("{}", y);  // prints 5
    *y = 6;  // error: cannot assign to immutable borrowed value
}
```

Mutable borrowing is done using the &mut operator. It allows you to use and modify a value, but it requires that you have exclusive access to the value for the duration of the borrow.

```rust, editable 
fn main() {
    let mut x = 5;
    let y = &mut x;  // y is a mutable borrow of x
    *y = 6;  // ok: we can modify x through y
    println!("{}", y);  // prints 6
}
```
