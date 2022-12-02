pub trait Greeting {
    fn say_hi(&self) {
        println!("Hi");
    }

    fn greet(&self);
}

struct Italian;
struct French;
struct English;

impl Greeting for Italian {
    fn greet(&self) {
        println!("Ciao");
    }
}

impl Greeting for French {
    fn greet(&self) {
        println!("Bonjour");
    }
}

impl Greeting for English {
    fn greet(&self) {
        println!("Hello");
    }
}

fn main() {
    Italian.say_hi();
    French.say_hi();
    English.say_hi();
    Italian.greet();
    French.greet();
    English.greet();
}
