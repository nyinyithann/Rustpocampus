use std::thread;

fn main() {
    let s = "Hello".to_owned();
    let mut v = vec![1, 2, 3];

    // the following code throws compile time error cuase
    // the thread can't borrow v
    // closure may outlive the current function
    // let handler = thread::spawn(|| println!("{:?}", v));
    // handler.join().unwrap();

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
    });

    dbg!(v);
}
