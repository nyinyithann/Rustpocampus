use std::fmt::{Display, Formatter, Result};

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display> Display for Point<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "x : {}, y : {}", self.x, self.y)
    }
}

fn main() {
    let p = Point::new(1.1, 2.2);
    let pstr = p.to_string();
    println!("{pstr}");
}
