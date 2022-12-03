# Trait Object
- A trait object is an opaque value of another type that implements an object safe base trait, its auto traits(`Send`, `Sync`, `Unpin`, `UnwindSafe`, `RefUnwindSafe`), and any supertraits of the base trait.
- Trait objects are written as the keyword `dyn` followed by a set of trait bounds - the first must be auto traits, and one life time if any. Paths to trait may be parenthesized. 

e.g -
- `dyn Trait`
- `dyn Trait + Send`
- `dyn Trait + Send + Sync`
- `dyn Trait + 'static`
- `dyn Trait + Send + 'static`
- `dyn Trait +`
- `dyn 'static + Trait.`
- `dyn (Trait)`

Due to the opaqueness of which concrete type the value is of, trait objects are dynamically sized types. Like all DSTs, trait objects are used behind some type of pointer; for example `&dyn SomeTrait` or `Box<dyn SomeTrait>`. Each instance of a pointer to a trait object includes:
- A pointer to an instance of a type T that implements SomeTrait
- A virtual method table, often just called a vtable, which contains, for each method of SomeTrait and its supertraits that T implements, a pointer to T's implementation (i.e. a function pointer).

> Object safe traits can be the base trait of a trait object [Object Safety, Object safe trait](https://doc.rust-lang.org/nightly/reference/items/traits.html#object-safety)

> Ref: [Rust Docs](https://doc.rust-lang.org/nightly/reference/types/trait-object.html)

**Returning a single trait using `impl`**
```rust, no_run, noplayground
{{#include ../../code/traits/trait_object/src/main.rs:init}}
```
⛔ **Rust won't allow this**
```rust, no_run, noplayground
fn get_runner(kind: i32) -> impl Runnable {
    if kind == 1 {
        Dog {}
    } else {
        Cat {}
    }
}
```
**Trait Object to rescue**
```rust, no_run, noplayground
{{#include ../../code/traits/trait_object/src/main.rs:tb}}
```

**full source code**
```rust
{{#include ../../code/traits/trait_object/src/main.rs:all}}
```

