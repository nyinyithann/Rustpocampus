# From

- Used for value-to-value conversion
- Implementing `From` automatically provides on with the implementation of [`Into`](https://doc.rust-lang.org/std/convert/trait.Into.html) via blanket implementation
- Conversion must not fail.
- Handy in error handling to encapsulate multiple error types into a single error type

#### `std::convert::From`
```rust
pub trait From<T>: Sized {
    fn from(value: T) -> Self;
}
```
`String` implements `From<&str>`
```rust 
let s = String::from("Rust");
println!("{s}");
```

`i32` implements `From<bool>`
```rust
let x = i32::from(true);
println!("{x}");
println!("{}", <bool as Into<i32>>::into(false));
```

#### Dollar to MMK conversion example
```rust
#[derive(Debug)]
struct MMK(f32);

#[derive(Debug)]
struct Dollar(f32);

impl From<Dollar> for MMK {
    fn from(value : Dollar) -> Self {
       MMK(value.0 * 5000.0)
    }
}

impl From<MMK> for Dollar {
    fn from(value : MMK) -> Self {
        Dollar(value.0 / 5000.0)
    }
}

fn main() {
    let mmk = MMK(5000_f32);
    let dollar = Dollar::from(mmk); 
    println!("{dollar:?}");

    let mmk = MMK(1000.); 
    let dollar : Dollar = mmk.into();
    println!("{dollar:?}");

    let mmk = MMK::from(Dollar(8400.));
    println!("{mmk:?}");
    let mmk : MMK = Dollar(1000.).into(); 
    println!("{mmk:?}");
}
```

#### Usage in Error Handling
```rust 
use std::fs;
use std::io;
use std::num;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError::IoError(error)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(error: num::ParseIntError) -> Self {
        CliError::ParseError(error)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    let mut contents = fs::read_to_string(&file_name)?;
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}
```
