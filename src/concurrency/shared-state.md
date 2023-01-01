# Shared-State Concurrency

### [`Mutext<T>`](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
- Need to lock it first before using 
- Need to unlock after using, `Drop` trait is implemented and unlocking will happen once Mutex goes out of scope


#### Example
```rust 
use std::thread;
use std::sync::{Mutex, Arc};

fn main() {
   let mut handlers  = vec![];  
    let counter = Arc::new(Mutex::new(0));
    for _ in 1..=10 {
        let counter = Arc::clone(&counter);
        handlers.push(thread::spawn(move || {
            let mut c = counter.lock().unwrap();
            println!("{:?}, Counter : {:?}", thread::current().id(), *c);
            *c += 1; 
        }));
    }

    for h in handlers {
        h.join().unwrap();
    }

    println!("Counter : {}", *counter.lock().unwrap());
}
```
