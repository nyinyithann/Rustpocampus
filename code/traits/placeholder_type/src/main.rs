// ANCHOR: all
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Dollar(f32);

#[derive(Debug, Copy, Clone)]
struct Cent(f32);

impl Add for Dollar {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl Add<Cent> for Dollar {
    type Output = Dollar;

    fn add(self, other: Cent) -> Self::Output {
        Self(self.0 + (other.0 / 100.))
    }
}

// ANCHOR: ph
fn add<T>(x: T, y: T) -> T
where
    T: Add<Output = T>,
{
    x + y
}
// ANCHOR_END: ph

fn main() {
    let d1 = Dollar(1.);
    let d2 = Dollar(2.);
    println!("{:?}", d1 + d2);
    println!("{:?}", d1 + Cent(120.0));

    println!("{:?}", add(d1, d2));
}
// ANCHOR_END: all
