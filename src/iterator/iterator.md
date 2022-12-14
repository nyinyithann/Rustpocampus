# [Iterator](https://doc.rust-lang.org/stable/std/iter/)

- A type that implements [`into-iterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) is __iterable__ and can be used with `for` loop syntax.
```rust
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator
    where
        <Self::IntoIter as Iterator>::Item == Self::Item;

    fn into_iter(self) -> Self::IntoIter;
}
```
- A type that implements [`iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) is __iterator__
```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

__iterator__ can be reterieved from __iterable types__ using the following 3 methods:
- `iter()` - iterates over `&T`
- `iter_mut()` - iterates over `&mut T`
- `into_iter()` - iterates over `T`


`iter()` and `iter_mut()` are not the methods of any trait.

#### Looping with `into_iter`
```rust, norun, noplayground
{{#include ../../code/iterator/src/bin/basic/main.rs:into_iter }}
```

#### Looping with `iter`
```rust, norun, noplayground
{{#include ../../code/iterator/src/bin/basic/main.rs:iter }}
```

#### Looping with `iter_mut`
```rust, norun, noplayground
{{#include ../../code/iterator/src/bin/basic/main.rs:iter_mut }}
```
