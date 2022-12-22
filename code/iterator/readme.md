# Notes

In toml file:
```
[[bin]]
name = "basic"
src = "src/bin/basic/main.rs"

[[bin]]
name = "tree"
src = "src/bin/tree/main.rs"
```

```
$ cargo run --bin basic

$ cargo run --bin tree
```

