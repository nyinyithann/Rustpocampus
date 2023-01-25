# Dereferencing

References are created explicitly with the `&` operator, and dereferenced explicitly with the `*` operator.

```rust 
    let x = 10;
    let rx = &x;
    assert!(*rx == 10);

    let mut y = 11;
    let ry = &mut y;
    *ry += 1; 
    assert!(*ry == 12);
```

The `.` operator implicitly dereferences its left operand if needed.

```rust
pub struct Person {
    name : &'static str,
    age : f32
}

let john_ref = &john;

assert!(john_ref.name == "John"); // implicitly dereference
assert!((*john_ref).name == "John"); // same as above, but with explicit dereference
assert!(john_ref.age == 40.3);
assert!((*john_ref).age == 40.3);
```

The `.` operator can also implicitly borrow a reference to its left operand, if needed for a method call.

```rust
let mut v = vec![10, 20, 3];
v.sort(); // implicitly borrow a mutable reference to v
(&mut v).sort(); // same as above
```
