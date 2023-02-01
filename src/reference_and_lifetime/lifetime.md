# Lifetime

A lifetime is a construct in Rust that represents the scope of a reference. The purpose of lifetimes is to ensure that references are always valid or to ensure a reference doesn't outlive its referent.

> Lifetimes are entirely figments of Rust's compile-time imagination. At runtime, a reference is nothing but an address; its lifetime is part of its type and has no runtime representation. -- The Programming Rust book

> The main aim of lifetimes is to prevent __dangling references__. -- The Book

#### Lifetime annotation ('a pronounce __tick a__)
`&i32`        // a reference without lifetime annotation<br/>
`&'a i32`     // a reference with explicit lifetime annotation<br/>
`&'a mut i32` // a mutable reference with explicit lifetime annotation

#### Dangling reference example
```rust 
// 'b is smaller than 'a and Rust rejects the program
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

The following functions return dangling references and won't compile.
```rust 
fn longest(fst : &str, snd: &str) -> &str {
    let string = "hello";
    return string.as_str();
}

// the returning lifetime is not related to the lifetime of paramaters
fn longest_2<'a>(fst : &str, snd: &str) -> &'a str {
    let string = "hello";
    return string.as_str();
}
```

#### Passing references and returning a reference from function
Rust sometimes needs explicit lifetime annotations for passing and returning references

```rust 
fn max<'a>(a : &'a i32, b : &'a i32) -> &'a i32 {
    if *a > *b {
        a
    } else {
        b
    }
}

// dangling pointer case here. won't compile
fn max_inner(a : &i32) -> &i32 {
    let b = 5;
    max(&a, &b)
}

fn main() {
    let x = 10;
    let y = 20;
    let result = max(&x, &y);
    println!("{result}");
   
    // occurs dangling pointer and won't compile
    let result = max_inner(&x);
    println!("{result}");
}

```
