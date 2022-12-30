use std::thread;

fn main() {

    let handler = thread::spawn(|| println!("{:?}", thread::current().id()));
    handler.join().unwrap();

    const NUM_OF_THREADS: usize = 10;
    let mut handlers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(NUM_OF_THREADS);
    for _ in 1..=10 {
        handlers.push(thread::spawn(|| println!("{:?}", thread::current().id())));
    }
    handlers.into_iter().for_each(move |h| h.join().unwrap());

    let handler = thread::Builder::new()
        .name("worker 1".to_string())
        .spawn(|| {
            let current = thread::current();
            println!("Name = {:?}, Id = {:?}", current.name().unwrap(), current.id());
        })
        .unwrap();
    handler.join().unwrap();

    println!("Main Thread ID = {:?}", thread::current().id());
}
