# Trait Object Lifetime

- If a trait object contain references, the lifetimes need to be expressed as part of it. e.g `Trait + 'a`
- Compiler infers lifetime most of the time based on [defaults](https://doc.rust-lang.org/nightly/reference/lifetime-elision.html#default-trait-object-lifetimes)

```rust, norun, noplayground
trait Foo { }

// these are the same because Box<T> has no lifetime bound on T
type T1 = Box<dyn Foo>;
type T2 = Box<dyn Foo + 'static>';
```

> more details [here](https://doc.rust-lang.org/nightly/reference/lifetime-elision.html#default-trait-object-lifetimes)
