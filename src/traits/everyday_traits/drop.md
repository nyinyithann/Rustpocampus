# Drop

By implementing 'Drop' trait, we can request Rust to run a piece of code when a value goes out of scope.

- 'Drop::drop' cannot be explicitly called
- To call destructor of a value explicitly, use 'std::mem::drop'
- 'Copy' and 'Drop' are exclusive. They cannot be implemented on the same type.

```rust 
# Drop trai
pub trait Drop {
    fn drop(&mut self);
}
```
#### Examples
```rust 
struct TheDrop;

impl Drop for TheDrop {
    fn drop(&mut self) {
        println!("Dropping TheDrop");
    }
}

struct TwoDrops {
    one : TheDrop,
    two : TheDrop,
}

impl Drop for TwoDrops {
    fn drop(&mut self) {
        println!("Dropping TwoDrops");
    }
}

fn main() {
   let _ = TwoDrops { one : TheDrop{}, two : TheDrop{} };

   let droppable = TheDrop{};
   std::mem::drop(droppable);

   use std::rc::Rc;
   let d1 = Rc::new(TheDrop{}); // doesn't print anything here
   let d2 = Rc::clone(&d1); 

    std::mem::drop(d1);
    std::mem::drop(d2);

   println!("Running...");
}
```


