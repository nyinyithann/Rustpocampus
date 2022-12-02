# Unit struct

Unit structs are most commonly used as marker. They have a size of zero bytes, but unlike empty enums they can be instantiated, making them isomorphic to the unit type (). Unit structs are useful when you need to implement a trait on something, but don’t need to store any data inside it.
<br/> -- From [here](https://doc.rust-lang.org/std/keyword.struct.html#:~:text=Unit%20structs%20are%20most%20commonly,store%20any%20data%20inside%20it)

```rust, ignore
struct Marker {} // use empty braces
struct Phontom;  // or just semicolon

// use same notation when creating an instance
let m = Marker {} ; 
let m = Marker; // throws error
let p = Phontom;
```

Realworld Usage of unit struct from [StackOverflow](shorturl.at/BJOQV)

```
Global
```
The global memory allocator, [Global](https://doc.rust-lang.org/std/alloc/struct.Global.html), is a unit struct:

```rust, ignore
pub struct Global;
```
It has no state of its own (because the state is global), but it implements traits like [Allocator](https://doc.rust-lang.org/std/alloc/trait.Allocator.html).

```
std::fmt::Error
```
The error for string formatting, [std::fmt::Error](https://doc.rust-lang.org/std/fmt/struct.Error.html), is a unit struct:

```rust, ignore
pub struct Error;
```
It has no state of its own, but it implements traits like [Error](https://doc.rust-lang.org/std/error/trait.Error.html).

```
RangeFull
```
The type for the .. operator, [RangeFull](https://doc.rust-lang.org/std/ops/struct.RangeFull.html), is a unit struct:

```rust, ignore
pub struct RangeFull;
```
It has no state of its own, but it implements traits like [RangeBounds](https://doc.rust-lang.org/std/ops/trait.RangeBounds.html).

__Crates__
```
chrono::Utc
```
The [Utc](https://docs.rs/chrono/0.4.19/chrono/offset/struct.Utc.html) timezone is a unit struct:

```
pub struct Utc;
```
It has no state of its own, but it implements traits like [TimeZone](https://docs.rs/chrono/0.4.19/chrono/offset/struct.Utc.html#impl-TimeZone) and is thus usable as a generic argument to ```Date``` and ```DateTime```.

