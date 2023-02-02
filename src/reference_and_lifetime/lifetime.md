# Lifetime

A lifetime is a construct in Rust that represents the scope of a reference. The purpose of lifetimes is to ensure that references are always valid or to ensure a reference doesn't outlive its referent.

> Lifetimes are entirely figments of Rust's compile-time imagination. At runtime, a reference is nothing but an address; its lifetime is part of its type and has no runtime representation. -- The Programming Rust book

> The main aim of lifetimes is to prevent __dangling references__. -- The Book

## Lifetime annotation ('a pronounce __tick a__)
`&i32`        // a reference without lifetime annotation<br/>
`&'a i32`     // a reference with explicit lifetime annotation<br/>
`&'a mut i32` // a mutable reference with explicit lifetime annotation

## Lifetime elision rules
Lifetime elision rules are a set of rules in Rust that allow the compiler to infer lifetimes in certain cases, without the need for explicit annotations.

- First Rule: Rust assigns a different lifetime paramater to each lifetime in each input type.
    - `fn foo(x : &i32)` becomes `fn foo<'a>(x : &'a i32)`
    - `fn foo(x : &i32, y : &i32)` becomes `fn foo<'a, 'b>(x : &'a i32, y : &'b i32)`
    - `fn foo(x : &ImportantExcerpt)` becomes `fn foo<'a>(x : &'a ImportantExcerpt)`
- Second Rule: If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`
- Third Rule: If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of self is assigned to all output lifetime parameters.

## Lifetimes on functions
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
Lifetime annotations need to be explicitly provided if Rust cannot infer lifetimes for input or output paramaters.

```rust 
fn max<'a>(a : &'a i32, b : &'a i32) -> &'a i32 {
    if *a > *b {
        a
    } else {
        b
    }
}

// this is OK 
fn max<'a>(a : &'a i32, b : &i32) -> &'a i32 {
    a
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

#### Lifetime with mutable references Example
2 lifetime annotations - one for mutable referenced container and one for the shared value - must be explicitly provided in the following example.
```rust
fn insert_str<'c, 'v>(source: &'c mut String, s : &'v str) {
    source.push_str(s);
}

fn insert_num<'c, 'v>(nums : &'c mut Vec<&'v i32>, num : &'v i32) {
    nums.push(num);
}

fn main() {
    let mut source = String::new();
    insert_str(&mut source, "hello");
    insert_str(&mut source, " world");
    println!("{:?}", source);

    let mut nums = Vec::new();
    insert_num(&mut nums, &10);
    insert_num(&mut nums, &11);
    println!("{:?}", nums);
}
```

## Lifetimes on Types 
Whenever a reference type appears inside another type's definition, you must write out its lifetime.
```rust
#[derive(Debug)]
struct FirstLast<'a> {
    first: &'a i32,
    second: &'a i32,
}

// no need to explicitly annotate lifetimes here 
// due to the first and second rule of lifetime elision rules
fn get_first_last(source: &[i32]) -> FirstLast {
    FirstLast {
        first : &source[0],
        second: &source[source.len() - 1],
    }
}

fn main() {
   let nums = vec![3,4,5,2,3,4,1,-2];
   let fl = get_first_last(&nums); 
   println!("{fl:?}");
}
```

Lifetime annotation on Enums.
```rust
enum MaybeString<'a> {
    Maybe(&'a str),
    Nothing
} 
```

## Lifetimes on Method definition
```rust 
#[derive(Debug)]
struct Excerpt<'a> {
    part : &'a str
}

impl<'a> Excerpt<'a> {
    fn new(part : &'a str) -> Self {
       Excerpt { part } 
    }

    fn display_and_return_part(&self) -> &str {
        println!("{self:?}");
        self.part
    }
}

fn main() {
    let s = "hello world";
    let expt = Excerpt::new(s.split(' ').next().unwrap());
    expt.display_and_return_part();
}
```

## The Static Lifetime (`'static`)

```rust 
let s : &'static str = "I'm static string"
```
`I'm static string` is stored directly in the program's binary which is located in static memory region.

