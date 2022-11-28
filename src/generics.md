# Generics

```rust, no_run, noplayground
{{#include ../code/generics/generic_point/src/main.rs:structs}}
```

### Generics in method definition
`T` declared just after `impl` will let Rust compiler identifies that the type paramater in the angle brackets in `Point` is a generic type rather than a concrete.

```rust, no_run, noplayground
{{#include ../code/generics/generic_point/src/main.rs:method_definition}}
```
Generic type paramaters in a struct can be different from the ones defined in struct declaration.
```rust, no_run, noplayground
{{#include ../code/generics/generic_point/src/main.rs:mixup}}
```

An `impl` block that only applies to a struct with a particular concrete type for the generic type paramater.

```rust, no_run, noplayground
{{#include ../code/generics/generic_point/src/main.rs:concrete_type}}
```

#### Runnable Code
```rust, editable
{{#include ../code/generics/generic_point/src/main.rs:all}}
```
