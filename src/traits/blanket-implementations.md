# Blanket Implementations

> "Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations." -- [The Book](https://rust-book.cs.brown.edu/ch10-02-traits.html)

```rust
{{#include ../../code/traits/blanket_impl/src/main.rs }}
```
`let pstr = p.to_string();` works because of the following blanket implementation in Rust Standard library.

```rust 
impl<T : Display> ToString for T { ... }
```

`ToString` trait is implemented in Standard library for any type that satisfies `Display` trait.
