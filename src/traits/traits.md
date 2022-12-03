# Traits

- Similiar to`interface` or `typeclass` or `modular-implicit` in other languages
- Trait defines shared behaviors in an abstract way.
- Trait enables paramateric and ad-hoc polymorphism in Rust.
- Trait consists of 3 kinds of associated item - functions, types, constants.
- All traits define an implicit type paramater `Self` that refers to "the type that is implementing this trait"
- Trait functions are not allowed to be `async` or `const`.


```rust,norun, noplayground
trait Example {
    const CONST_WITH_NO_DEFAULT : i32,
    const CONST_WITH_WITH_DEFAULT : i32 = 10,
    type item,
    fn method_without_default(&self);
    fn method_with_default(&self) { }
}
```

>_orphan rule_ dictates that external traits cannot be implemented for external types to ensure code doesn't break if two crates provide conflicting implementations.

#### A simple trait implementation
- To implement a trait, use `impl Trait for Type` block
- One impl block per type per trait

```rust
{{#include ../../code/traits/traits_basic/src/main.rs}}
```
