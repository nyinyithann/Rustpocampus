# Implemting Iterator

If a type is implemented `iterator`, `into_iterator` is implemented automatically for it because of the following [blanket implementation](https://doc.rust-lang.org/src/core/iter/traits/collect.rs.html#266):
``` rust
impl<I: ~const Iterator> const IntoIterator for I {
    type Item = I::Item;
    type IntoIter = I;

    #[inline]
    fn into_iter(self) -> I {
        self
    }
}
```

#### Sample Iterator implementation
```rust
{{#include  ../../code/iterator/src/bin/implement-iterator/main.rs }}
```
