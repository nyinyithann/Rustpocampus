trait Duck {
    fn walk(&self) {
        println!("walking like a duck.")
    }
}

trait Chicken {
    fn walk(&self) {
        println!("walking like a chicken.")
    }
}

struct Person {
    name: String,
}

impl Person {
    fn walk(&self) {
        println!("walking like a human.")
    }
}

impl Duck for Person {}
impl Chicken for Person {}

fn main() {
    let p = Person {
        name: String::from("Ryan"),
    };

    println!("My name is {}. And I am ", p.name);
    p.walk();

    Duck::walk(&p);
    Chicken::walk(&p);

    <Person as Duck>::walk(&p);
    <Person as Chicken>::walk(&p);
}
