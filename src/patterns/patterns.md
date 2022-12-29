# Patterns
Patterns are used to match values against structures and to, optionally, bind variables to values inside these structures. They are also used in variable declarations and parameters for functions and closures. [-- Ref](https://doc.rust-lang.org/reference/patterns.html#path-patterns)

#### Refutability
A pattern is said to be refutable when it has the possibility of not being matched by the value it is being matched against. Irrefutable patterns, on the other hand, always match the value they are being matched against. Examples:

```rust
let (x, y) = (1, 2);               // "(x, y)" is an irrefutable pattern

if let (a, 3) = (1, 2) {           // "(a, 3)" is refutable, and will not match
    panic!("Shouldn't reach here");
} else if let (a, 4) = (3, 4) {    // "(a, 4)" is refutable, and will match
    println!("Matched ({}, 4)", a);
} 
```

#### Binding modes
To service better ergonomics, patterns operate in different binding modes in order to make it easier to bind references to values. When a reference value is matched by a non-reference pattern, it will be automatically treated as a ref or ref mut binding. Example:

```rust
let x: &Option<i32> = &Some(3);
if let Some(y) = x {
    // y was converted to `ref y` and its type is &i32
}
```

If a binding pattern does not explicitly have ref, ref mut, or mut, then it uses the default binding mode to determine how the variable is bound. The default binding mode starts in "move" mode which uses move semantics.

#### Patterns
```rust
fn main() {
    // 1. Wildcard Pattern
    // _ (an underscore symbol) is used for wildcard pattern, and it matches any value
    let (a, _) = (1, 2);
    assert_eq!(a, 1);

    match a {
        _ => println!("a can be any number"),
    }

    // 2. Literal Pattern : integer literal
    for i in -3..2 {
        match i {
            -3 | -2 | -1 => println!("Negative Number"),
            0 => println!("Zero"),
            _ => println!("Positive Number"),
        }
    }
    // Literal Pattern: char literal
    for a in 'a'..'z' {
        match a {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("It's vowel"),
            _ => println!("It's consonant"),
        }
    }

    // 3. Range Pattern
    let ch = 'a';
    match ch {
        'a'..='z' => println!("lowercase letter"),
        'A'..='Z' => println!("uppercase letter"),
        _ => println!("other"),
    }

    // 4. Identifier Pattern
    let x = 16;
    match x {
        // bind x to e, print e if it is between 1 and 10 inclusively
        e @ 1..=10 => println!("{e}"),
        // bind x to e, print e if it is greater than 10
        e @ 11.. => println!("{e} is greater than 10."),
        e => println!("{e}"),
    }

    // this is also an identifier pattern, bind "hello" to s1
    let s1 = "hello";
    println!("{s1}");
    // same as above, bind "hello" to s using match
    match "hello" {
        s => println!("{s}"),
    }

    // Identifier Pattern with ref or mut
    let z = "Zan".to_string();
    match z {
        e => println!("{e}"),
    }
    // println!("{z}"); // compile time error cause z is moved to e and no longer valid here

    // with ref
    let z = "Zan".to_string();
    match z {
        ref e => println!("{e}"),
    }
    assert_eq!(z, "Zan");

    // with ref mut
    let mut z = "Zan".to_string();
    match z {
        ref mut e => *e = "Ryan Zan".to_string(),
    }
    assert_eq!(z, "Ryan Zan");

    // 5. Slice Pattern for fixed-sized array
    let odds = [1, 2, 3, 4, 5];
    match odds {
        [1, 2, a, _, _] => {
            println!("Starts with 1 and 2");
            println!("The third is {a}")
        }
        _ => println!("Starts with something else"),
    }

    // Slice Pattern for slices of dynamic size
    let evens = vec![2, 4, 6];
    match evens[..] {
        [a, b] => println!("{a}, {b}"),
        [a, b, c] => println!("{a}, {b}, {c}"),
        _ => println!("Not Matched"),
    }

    // 6. Tuple Pattern
    let (i, s) = (7, "monkeys");
    assert_eq!(i, 7);
    assert_eq!(s, "monkeys");

    // 7. Tuple Struct Pattern
    struct Point(f32, f32);
    let p = Point(1.1, 2.2);
    match p {
        Point(x, y) => println!("{x}, {y}"),
    }

    // 8. Rest Pattern - only be used in tuple, tuple struct, and slice pattern
    let odds = (1, 3, 5, 7, 9, 11);
    let (fst, ..) = odds;
    assert_eq!(fst, 1);
    let (.., last) = odds;
    assert_eq!(last, 11);

    let evens = vec![2, 4, 6, 8];
    fn sum(slice: &[i32], acc: i32) -> i32 {
        match slice {
            [] => acc,
            [h, tail @ ..] => sum(&tail, h + acc),
        }
    }
    let r = sum(&evens, 0);
    assert_eq!(r, evens.iter().sum());

    // 9. Reference Pattern
    let one = &1;
    match one {
        &1 => println!("yes"),
        _ => println!("hmm"),
    }

    let maybe: &Option<i32> = &Some(10);
    match maybe {
        &Some(x) => println!("{x}"),
        &None => println!("Nothing"),
    }

    // or simply like below (check binding modes section)
    let some_val = &Some(10);
    match some_val {
        Some(x) => println!("{x}"),
        None => println!("None"),
    }

    // 10. Grouped Pattern
    let two = &2;
    match two {
        &(1..=5) => println!("Between 1 and 5"),
        &(6..=10) => println!("Between 6 and 10"),
        _ => println!("Other"),
    }

    // 11. Path Pattern with enum
    enum Color {
        Red,
        Green,
        Blue,
    }
    let red = Color::Red;
    match red {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
    }

    // Path Pattern with constant
    const MAX_VALUE: i32 = 100;

    let x = 75;
    match x {
        MAX_VALUE => println!("The value is the maximum"),
        _ => println!("The value is something else"),
    }

    // 12. Struct Pattern
    struct Location {
        x: i32,
        y: i32,
    }
    let p1 = Location { x: 1, y: 2 };
    match p1 {
        Location { x: 1, y: 3 } => println!("A"),
        Location { x: 2, y: 2 } => println!("B"),
        Location { .. } => println!("Unknown"),
    }

    // 13. Macro Invocation Pattern
    let _v = vec![1,2]; // this is macro invocation patter - invoke vec macro

    // another simple example
    macro_rules! foo {
        ($x:expr) => {
            $x
        };
    }

    let x = 5;
    match x {
        foo!(5) => println!("It's 5"),
        _ => println!("The value is something else"),
    }
}
```
