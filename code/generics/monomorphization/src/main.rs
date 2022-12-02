fn id<T>(x: T) -> T {
    x
}

fn main() {
    let int_id = id(10);
    let f32_id = id(1.1_f32);
    let string_id = id("hello");
    println!("{}", int_id);
    println!("{}", f32_id);
    println!("{}", string_id);
}
