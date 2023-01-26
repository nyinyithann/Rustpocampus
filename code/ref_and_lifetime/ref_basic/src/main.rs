pub struct Person {
    name: &'static str,
    age: f32,
}

fn main() {
    let x = 10;
    let rx = &x;
    assert!(*rx == 10);

    let mut y = 11;
    let ry = &mut y;
    *ry += 1;
    assert!(*ry == 12);

    let john = Person {
        name: "John",
        age: 40.3,
    };
    let john_ref = &john;

    assert!(john_ref.name == "John");
    assert!((*john_ref).name == "John");
    assert!(john_ref.age == 40.3);
    assert!((*john_ref).age == 40.3);

    let name : &'static str = "ryan";
    let mut ret = String::new();
    for x in name.chars().rev() {
       ret.push(x); 
    } 
}
