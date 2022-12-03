use std::{fmt::Display, ops::Add};

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// conditionally implement methods inside this impl block
impl<T: Add<Output = T> + Copy + Display> Pair<T> {
    fn double_display(&self) {
        let dp = Pair::new(self.x + self.x, self.y + self.y);
        println!("x = {}, y = {}", dp.x, dp.y);
    }
}

fn main() {
    let p = Pair::new(1.1, 2.2);
    p.double_display();

    // the following won't work, compile time error
    // let pstr = Pair::new("hello".to_string(), "world".to_string());
    // pstr.double_display();
}
