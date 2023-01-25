# Reference & lifetime

Rust has two type of pointers mainly:
- Owning pointers e.g. `Box<T>` - When the owner is dropped, the referent goes with it.
- Nonowning pointers a.k.a _references_ - references must never outlive their referents.

There are two kinds of Nonowning pointers or references:
- Shared reference: `&`
- Mutable reference: `&mut`

Which obey the following rules:
- A reference cannot outlive its referent
- A mutable reference cannot be shared

#### Shared Reference
- Rust allows multiple shared references
- Rust doesn't allow shared referencees to mutate their referents
- Shared references must not outlive theire referents
- Shared references are [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html)
- `&e` is a shared reference to e's value; if `e` has type `T`, then `&e` has the type `&T` pronounced `ref T`

#### Mutable Reference
- With a mutable reference to a value, it can be read or modified
- Rust allow only one mutable reference to a value at a time
- `&mut e` is a mutable reference to e's value; if `e` has type `T`, then `&mut e` has the type `&mut T` pronounced `ref mute T`
