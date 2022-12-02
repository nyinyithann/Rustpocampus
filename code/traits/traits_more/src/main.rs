use std::ops::Add;

fn double<T>(x : impl Add<Output=T> + Copy + Clone) -> T {
    x + x
}


// fn add<T>(x : impl Add<Output=T>, y : impl Add<Output=T>) -> T {
//     x + y
// }

fn main() {
    println!("{}", double(1));
}
