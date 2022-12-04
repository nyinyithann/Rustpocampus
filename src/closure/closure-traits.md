# Fn, FnOnce, FnMut
Closures are backed by [`Fn`](https://doc.rust-lang.org/std/ops/trait.Fn.html), [`FnOnce`](https://doc.rust-lang.org/std/ops/trait.FnOnce.html) and [`FnMut`](https://doc.rust-lang.org/std/ops/trait.FnMut.html) traits.

```rust
pub trait Fn<Args>: FnMut<Args> {
    extern "rust-call"
    fn call(&self, args: Args) -> Self::Output;
}

pub trait FnMut<Args>: FnOnce<Args> {
    extern "rust-call" 
    fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait FnOnce<Args> {
    type Output;

    extern "rust-call" 
    fn call_once(self, args: Args) -> Self::Output;
}
```

#### `Fn` Trait
- can be called multiple times
- don't move captured values out of its body
- don't mutate captured values
- even might not capture any environment variables
- `FnMut` and `FnOnce` are supertraits of `Fn` and  it can be used as a paramater where a `FnMut` or `FnOnce` is expected
- is implemented automatically for closures which take immutable references to environment variables or don't capture anything at all

```rust
fn apply<F>(x : i32, invoker : F ) -> i32
where F : Fn(i32) -> i32 {
  invoker(x)        
}

fn main() {
  let double = |x : i32| x * 2; 
  println!("{:?}", apply(10, double));
}
```
#### `FnOnce` Trait
- cannot be called multiple times
- is implemented automatically by closures that might consume the captured virables
  or moves the captured variables out of its body

```rust
fn apply<F>(invoker : F ) 
where F : FnOnce() -> String {
  println!("{:?}", invoker());
}

fn main() {
  let magic_string = String::from("a kind of magic");
  let closure = move || magic_string ;
  apply(closure); 
 
  // closure cannot be called again
  // apply(closure); 
}
```
#### `FnMut` Trait
- can be called multiple times
- don't move the captured variables out of closure body, but mutate them
```rust
fn apply<F>(mut invoker : F ) 
where F : FnMut(&str) -> () {
  invoker("abra");
}

fn main() {
  let mut magic_string = String::from("a kind of magic");
  let closure = |x: &str| { magic_string.push_str(x); println!("{:?}", magic_string); } ;
  apply(closure); 
}
```

#### Returning Closure
```rust
fn closure_factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
  Box::new(move |y : i32| x + y)
} 

fn main() {
  let c = closure_factory(10);
  println!("{:?}", c(20));
}
```

