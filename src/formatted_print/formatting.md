# Formatted Print

[`std::fmt`](https://doc.rust-lang.org/std/fmt/#macros) has utilities for formatting and printing `Strings`. Some of which include:

* `format!`: write formatted text to `String`
* `print!`: same as `format!` but the text is printed to the console
  (io::stdout).
* `println!`: same as `print!` but a newline is appended.
* `eprint!`: same as `print!` but the text is printed to the standard error
  (io::stderr).
* `eprintln!`: same as `eprint!` but a newline is appended.
* `write!`: emit the format string to a specified stream.
* `writeln!` same as `write!` but a newline is appended

#### Positional paramaters
```rust
println!("{1} {} {0} {}", 1, 2); // => 2 1 1 2

let formatted : String = format!("{1} {} {0} {}", 1, 2);
println!("{formatted}"); // => 2 1 1 2
```

#### Named paramaters
```rust 
let seven = 7;
println!("{one} {two} ... {seven}", one = 1, two = 2);
```

#### Using `write!` of `std::io::Write` and `std::fmt::Write`
```rust, ignore, editable, mdbook-runnable
use std::io::{self, Write as _};
use std::fmt::Write as _;

fn main() -> io::Result<()> {
    
    write!(&mut io::stdout(), "Hello, {}!", "World")?;

    let mut vec = Vec::new();
    write!(&mut vec, "Hello, {}!", "World")?;
    assert_eq!(vec, b"Hello, World!");

    let mut s = String::new();
    write!(&mut s, "Life is {}", "Bootiful"); // std::fmt::Write
    assert_eq!(s, "Life is Bootiful");
    
    Ok(())
}
```

#### Using [`format_args!`](https://doc.rust-lang.org/std/fmt/#format_args)
- result is of type `fmt::Arguments`
- result can be passed around
- no heap allocation

```rust, ignore, editable, mdbook-runnable
use std::io::{self, Write};
use std::fmt::{self};

fn write_error_log(arg: fmt::Arguments) -> std::io::Result<()>{
    writeln!(&mut io::stdout(), "{}", arg )?;
    Ok(())
}

fn main() -> io::Result<()> {
    write_error_log(format_args!("Error number is {}.", 1))?;
    Ok(())
}
```


