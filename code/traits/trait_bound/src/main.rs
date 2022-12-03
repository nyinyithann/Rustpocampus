// ANCHOR: all
use std::{fmt::Debug, fmt::Display, ops::Add};

// ANCHOR: impl
fn double<T>(x: impl Add<Output = T> + Copy) -> T {
    x + x
}
// ANCHOR_END: impl

// ANCHOR: traitbound
fn double_2<T: Add<Output = T> + Copy>(x: T) -> T {
    x + x
}

fn add<T: Add<Output = T>>(x: T, y: T) -> T {
    x + y
}
// ANCHOR_END: traitbound

// ANCHOR: tb-where
fn double_3<T>(x: T) -> T
where
    T: Add<Output = T> + Copy,
{
    x + x
}

fn add_2<T>(x: T, y: T) -> T
where
    T: Add<Output = T>,
{
    x + y
}

fn foo<A, B>(x: A, y: B) -> i32
where
    A: Display + Clone,
    B: Debug + Clone,
{
    unimplemented!()
}
// ANCHOR_END: tb-where

fn main() {
    println!("{}", double(1));
    println!("{}", double(1.1));
    println!("{}", double_2(10));
    println!("{}", double_3(20));
    println!("{}", add(10, 2));
    println!("{}", add_2(10, 2));
}
// ANCHOR_END: all
