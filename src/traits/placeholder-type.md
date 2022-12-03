# Associated Type

`std::ops::Add` trait has the folowing implementation. `<Rhs = Self>` is called __default type paramaters__. `Output` is called __placeholder or associated type__.

```rust, norun, noplayground
pub trait Add<Rhs = Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

##### Usage sample of `Add` trait's placeholder type
```rust, norun, noplayground
{{#include ../../code/traits/placeholder_type/src/main.rs:ph}}
```

#### sample implementation of `Add` trait
```rust 
{{#include ../../code/traits/placeholder_type/src/main.rs:all}}
```

