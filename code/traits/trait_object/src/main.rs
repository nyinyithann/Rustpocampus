// ANCHOR: all
// ANCHOR: init
pub trait Runnable {
    fn run(&self);
}

struct Dog;
struct Cat;

impl Runnable for Dog {
    fn run(&self) {
        println!("The dog is running.")
    }
}

impl Runnable for Cat {
    fn run(&self) {
        println!("The cat is running.")
    }
}

fn get_running_dog() -> impl Runnable {
    Dog {}
}
// ANCHOR_END: init

// but we can't do this in Rust
/*
fn get_runner(kind: i32) -> impl Runnable {
    if kind == 1 {
        Dog {}
    } else {
        Cat {}
    }
}
*/

// ANCHOR: tb
fn get_runner_dyn(kind: i32) -> &'static dyn Runnable {
    if kind == 1 {
        &Dog {}
    } else {
        &Cat {}
    }
}

fn get_runner_box(kind: i32) -> Box<dyn Runnable> {
    if kind == 1 {
        Box::new(Dog {})
    } else {
        Box::new(Cat {})
    }
}

fn invoke_runner_dyn(runner: &dyn Runnable) {
    runner.run();
}

fn invoke_runner_box(runner: Box<dyn Runnable>) {
    runner.run();
}
// ANCHOR_END: tb

fn main() {
    get_running_dog().run();
    get_runner_dyn(2).run();
    get_runner_box(1).run();

    invoke_runner_dyn(&Dog {});
    invoke_runner_box(Box::new(Dog {}));
}
// ANCHOR_END: all
