# Traits

- Similiar to`interface` or `typeclass` or `modular-implicit` in other languages
- Trait defines shared behaviors in an abstract way.
- Trait enables paramateric and ad-hoc polymorphism in Rust.

>_orphan rule_ dictates that external traits cannot be implemented for external types to ensure code doesn't break if two crates provide conflicting implementations.

#### A simple trait implementation
```rust
{{#include ../../code/traits/traits_basic/src/main.rs}}
```
