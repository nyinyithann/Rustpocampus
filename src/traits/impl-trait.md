# impl trait

`impl Trait` can appear in two places:
    - argument position: anonymous type paramater
    - return position : abstract return type

```rust, norun, noplayground
trait Trait {}

fn foo(arg : impl Trait) { }

fn bar() -> impl Trait { }
```

**Generic type paramater vs impl Trait**

```rust, norun, noplayground
trait Trait {}

fn foo_generic<T: Trait>(arg : T) { }
fn foo_impl(arg : impl Trait) { }
```
> `foo_generic` can do like `foo_generic::<i32>(1)` but `foo_impl` can't

```rust 
fn bar_generic<T: Trait> -> T { }
fn bar_impl() -> impl Trait { }
```
> `bar_generic` allows the caller to determine the return type `T`, but `bar_impl` won't

**impl Trait in return position**

```rust, norun, noplayground
// instead of this
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
  Box:new(|x| x + 1)
}

// we can do this with impl trait to avoid heap allocation and dynmic dispatch
fn returns_closure() -> impl Fn(i32) -> i32 {
  |x| x + 1
}
```

#### Limitations
`impl Trait` can appear in a free or inherent function 
 - as a paramater
 - as a return type 

It can't appear 
 - inside implementation of traits
 - let binding
 - inside a type alias

#### &impl Trait
```rust, editable
#![recursion_limit="1000"] 
use std::fmt::Display;

enum Node<T> {
    Leaf(T),
    Children(Vec<Node<T>>),
}

impl<T: Display> Node<T> {
    fn traverse(&self, f: &impl Fn(&T)) {
        match self {
            Node::Leaf(x) => f(x),
            Node::Children(children) => {
                for n in children {
                    n.traverse(f);
                }
            }
        }
    }
}

fn main() {
    let tree = Node::Children(vec![
        Node::Leaf(1),
        Node::Children(vec![Node::Leaf(2), Node::Leaf(3), Node::Leaf(4)]),
    ]);
    tree.traverse(&|x| println!("{x}"))
}
```
