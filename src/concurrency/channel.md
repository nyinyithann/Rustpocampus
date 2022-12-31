# Channel

[`std::sync::mpsc`](https://doc.rust-lang.org/std/sync/mpsc/) provides multi-producer, single-consumer FIFO queue communication primitives and message-based communication over channels.
 
#### Example code (multiple producers and one receiver)
```rust 
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
   
    let handler1 = thread::spawn(move || {
        for i in 1..=10 {
            tx.send(format!("{} {}", "T1: Hi", i)).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
        
    });

    let handler2 = thread::spawn(move || {
        for i in 1..=10 {
            tx1.send(format!("{} {}", "T2: Hi", i)).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("{received:?}");
    }

    handler2.join().unwrap();
    handler1.join().unwrap();
}
```
