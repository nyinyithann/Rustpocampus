trait Walkable {
    fn walk(&self, steps: i32);
}

trait Talkable {
    fn talk(&self, times: i32);
}

trait Human: Walkable + Talkable {
    fn laugh(&self, times: i32);
}

struct Person;

impl Walkable for Person {
    fn walk(&self, steps: i32) {
        println!("I walk {steps} stpes.");
    }
}

impl Talkable for Person {
    fn talk(&self, times: i32) {
        println!("I talk {times} times.");
    }
}

impl Human for Person {
    fn laugh(&self, times: i32) {
        println!("I laugh {times} times.");
    }
}

fn act_like_a_human(bloke : &dyn Human) {
    bloke.walk(10);
    bloke.laugh(10);
    bloke.talk(10);
}

fn main() {
    act_like_a_human(&Person{});
}
