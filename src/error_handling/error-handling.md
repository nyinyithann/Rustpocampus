# Error Handling

- recoverable error such as file not found, use `Result<T, E>`
- unrecoverable error such as accessing a location beyond the end of array, use `panic!`

- you can switch from unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate [profile] sections in your Cargo.toml file
    ```
    [profile.release]
    panic = 'abort'
    ```

#### [Result type](https://doc.rust-lang.org/std/result/enum.Result.html)
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

> Instead of `unwrap`, `expect` should be used to give more context about the operation<br/>
>```rust 
>let file = std::fs::File::open("hello.txt")
>            .expect("hello.txt should be included in the project")
>```

> ? operator can be used with `Result` or `Option` type, but can't mix can match between them
```rust
// e? expression, where e is of type Result<T, E>,  equals to
match e {
  Ok(x) => x,
  Err(err) => { return Err(err); }
}
```

#### Sample Code
```rust, noedit
{{#include ../../code/error_handling/fileopen/src/main.rs:all }}
```
