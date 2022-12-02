# Tuple struct

Fields of a tuple struct can be accessed using implicit field names (0,1,...)

```rust, editable
struct RGB(i32, i32, i32);

fn main() {
    let c = RGB(1, 255, 255);
    println!("R={}, G={}, B={}", c.0, c.1, c.2);

    /* patern match */
    let RGB(r, g, b) = c;
    println!("{r}, {g}, {b}");

    if let RGB (_, g,_ ) = c {
        println!("G is {g}.");
    }
}
```

Tuple struct can be used for [Newtype](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types) pattern.

```rust, editable
use std::fmt::{Display, Formatter, Result};
use std::ops::Add;

struct Pound(f32);
struct Kilogram(f32);

impl Add<Pound> for Kilogram {
    type Output = Kilogram;

    fn add(self, rhs: Pound) -> Self::Output {
        Kilogram(self.0 + (rhs.0 / 2.2))
    }
}

impl Display for Kilogram {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} Kg", self.0)
    }
}

fn main() {
    let weight = Pound(198.);
    let zero =Kilogram(0.);
    println!("weight: {}", zero + weight);
}
```
