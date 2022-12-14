use std::option::IterMut;

#[derive(Debug)]
struct Counter {
    count : i32
}

impl Counter {
    fn new () -> Self {
        Self { count : 0}
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count == 17 {
            None
        } else {
            Some(self.count)
        }
    }
}

fn main() {
    let counter = Counter::new();

    for i in counter {
        println!("{i}");
    }

}
