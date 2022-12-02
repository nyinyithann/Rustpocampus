# Monomorphization
"Compile-time process where polymorphic functions are replaced by many monomorphic functions for each unique instantiation."
-- [Wikipedia](https://en.wikipedia.org/wiki/Monomorphization?oldformat=true)

#### Before monomorphization
```rust
{{#include ../../code/generics/monomorphization/src/main.rs}}
```

#### After Monomorphization
Rust might generate monomorphic functions with different naames.

```rust
fn id_i32(x: i32) -> i32 {
    x
}

fn id_f32(x: f32) -> f32 {
    x
}

fn id_str(x: &str) -> &str {
    x
}

fn main() {
    let int_id = id_i32(10);
    let f32_id = id_f32(1.1_f32);
    let string_id = id_str("hello");
    println!("{}", int_id);
    println!("{}", f32_id);
    println!("{}", string_id);
}
```
