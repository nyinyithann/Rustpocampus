# Thread

- A thread has its own thread-local stack. The default size of a spawned thread is 2 MiB (Mebibyte 2<sup>20</sup>) - subject to change.
- A thread can be spawned by calling `std::thread::spawn`
    ```rust 
    pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    ```

#### Spawning a thread
```rust 
use std::thread;

fn main() {
    let handler = thread::spawn(|| println!("{:?}", thread::current().id()));
    // wait until the spawned thread finishes its job
    handler.join().unwrap();
}
```

#### Create a thread with Builder and configure stack size
- the stack size of the main thread is not determined by Rust
- can set stack size via RUST_MIN_STACK env variable but `Builder::stack_size` will override it
```rust 
use std::thread;
fn main() {
    let handler = thread::Builder::new()
        .stack_size(1024 /* bytes */)
        .name("worker 1".to_string())
        .spawn(|| {
            let current = thread::current();
            println!("Name = {:?}, Id = {:?}", current.name().unwrap(), current.id());
        })
        .unwrap();
    handler.join().unwrap();
    println!("Main Thread ID = {:?}", thread::current().id());
}
```

#### Spawning many threads
```rust
use std::thread;

fn main() {
    const NUM_OF_THREADS: usize = 10;
    let mut handlers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(NUM_OF_THREADS);
    for _ in 1..=10 {
        handlers.push(thread::spawn(|| println!("{:?}", thread::current().id())));
    }
    handlers.into_iter().for_each(move |h| h.join().unwrap());

    println!("Main Thread ID = {:?}", thread::current().id());
}
```

#### Scoped Thread [Ref](https://medium.com/@KevinBGreene/async-programming-in-rust-part-2-diving-into-scoped-threads-50aace437756)
- non-scoped threads __cannot__ borrow non-`'static` value 
- scoped threads __can__ borrow non-`'static` value as the scope guarantees all threads will be joined at the end of the scope

```rust 
use std::thread;

fn main() {
    let  v = vec![1, 2, 3];

    // the following code throws compile time error cuase
    // the thread can't borrow v
    // closure may outlive the current function
    let handler = thread::spawn(|| println!("{:?}", v));
    handler.join().unwrap();
}
```

With scoped thread:
```rust
use std::thread;
fn main() {
    let s = "Hello".to_owned();
    let mut v = vec![1, 2, 3];
    
    thread::scope(|scope| {
        scope.spawn(|| {
            println!("soped thread borrows s");
            dbg!(&s);
        });
        
        scope.spawn(|| {
            println!("soped thread mutably borrows v");
            v.push(4); 
            v.push(5); 
        });
        
        // all threads join at  the end of the scope
    });

    println!("{v:?}"); // => [1, 2, 3, 4, 5]
}
```
