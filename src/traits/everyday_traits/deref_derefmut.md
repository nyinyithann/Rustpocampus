# Deref & DerefMut
Used for immutable dereferencing operations, like *v.

In addition to being used for explicit dereferencing operations with the (unary) * operator in immutable contexts, Deref is also used implicitly by the compiler in many circumstances.

> Deref should only be implemented for smart pointers to avoid confusion.

Rust does __deref coercion__ when it finds types and trait implementations in three cases:
- From `&T` to `&U` when T: Deref<Target=U>
- From `&mut` T to `&mut` U when T: DerefMut<Target=U>
- From `&mut` T to `&U` when T: Deref<Target=U>

```rust
pub trait Deref {
    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}

pub trait DerefMut: Deref {
    fn deref_mut(&mut self) -> &mut Self::Target;
}
```

#### Deref implementation
```rust 
#[derive(Debug)]
struct SmartBox<T> {
   value : T 
}

impl<T> SmartBox<T> {
    fn new(v : T) -> Self {
            Self{ value : v}
    }
}

use std::ops::Deref;
use std::ops::DerefMut;
impl<T> Deref for SmartBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for SmartBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

fn main() {
   let mut sb = SmartBox::new(10);
   println!("{}", *sb);
   *sb = 20;
   println!("{}", *sb);

   let mut name = SmartBox::new("Ryan");
   *name = "Ryan Zan";
   println!("{}", *name);
   
   let mut name = SmartBox::new(String::from("Hello"));
   (*name).push_str(" Zan Thumyat");
   println!("{}", *name);
}
```
