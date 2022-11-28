/* ANCHOR: all */
// ANCHOR: structs
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2D<A, B> {
    x: A,
    y: B,
}
// ANCHOR_END: structs

// ANCHOR: method_definition
impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<A, B> Point2D<A, B> {
    fn new(x: A, y: B) -> Point2D<A, B> {
        Point2D { x, y }
    }

    fn x(&self) -> &A {
        &self.x
    }

    fn y(&self) -> &B {
        &self.y
    }
}
// ANCHOR_END: method_definition

// ANCHOR: mixup
impl<X1, Y1> Point2D<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2D<X2, Y2>) -> Point2D<X1, Y2> {
        Point2D {
            x: self.x,
            y: other.y,
        }
    }
}
// ANCHOR_END: mixup

// ANCHOR: concrete_type
impl Point<f32> {
    fn distance(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
// ANCHOR_END: concrete_type

fn main() {
    let p = Point::new(10.10, 20.20);
    println!("{:?}", p);
    println!("{:?}", p.x());
    println!("{:?}", p.distance());

    let p_2d = Point2D::new(1, 2.2);
    println!("{:?}", p_2d);
    println!("x = {:?}", p_2d.x());
    println!("y = {:?}", p_2d.y());

    let mixed_point = p_2d.mixup(Point2D::new(1000.0, 2000.0));
    print!("{:?}", mixed_point);
}
/* ANCHOR_END: all */
