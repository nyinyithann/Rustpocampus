# [struct](https://doc.rust-lang.org/std/keyword.struct.html#:~:text=Unit%20structs%20are%20most%20commonly,store%20any%20data%20inside%20it.)

__3 types of struct__
- __regular (C-like) struct__
```rust
# #[derive(Debug)]
struct Person {
    name : String,
    age : f32
}

fn main() {
    let ryan = Person { name : "Ryan".to_string(), age : 6.3 };
    println!("{:?}", ryan);

    /* field punning */
    let name = "Ryan".to_string();
    let age = 6.3;
    let ryan = Person { name, age }; // <-
    println!("{:?}", ryan);

    /* with pattern match */
    let Person { name, age } = ryan;
    println!("{name}, {age}");

    /* spreading, assign fields */
    let Person { name : n, age : a } = 
        Person { name : "Ryan Clone".to_string(), .. ryan } ;
    println!("{n}, {a}");
}
```
- __tuple struct__
```rust
struct Person(String, f32);
```
- __unit struct__
```rust
struct Any;
struct Never;
struct Unit;
```

__A struct with some methods__
```rust
# extern crate chrono;
use chrono::format::ParseResult;
use chrono::prelude::*;
use std::fmt;

struct Lang {
    name: String,
    type_system: String,
    created_at: ParseResult<NaiveDate>,
}

impl Lang {
    fn new(name: &str, type_system: &str, created_at: &str) -> Lang {
        Lang {
            name: name.to_string(),
            type_system: type_system.to_lowercase(),
            created_at: NaiveDate::parse_from_str(created_at, "%d/%m/%Y"),
        }
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let d = if let Ok(d) = self.created_at {
            d.format("%d/%m/%Y").to_string()
        } else {
            "".to_string()
        };

        write!(
            f,
            "name : {}, type_system : {}, created_at : {}",
            self.name, self.type_system, d
        )
    }
}

fn main() {
    let rust = Lang::new("Rust", "Strong, Static", "1/1/2010");
    println!("{rust}");
}
```
