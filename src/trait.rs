use std::fmt::{Display, Formatter, Error};

fn main() {
    let c = Circle{radius: 2f64};
    println!("{:?}", c);
    println!("{:#?}", c);
    let c = Box::new(c) as Box<dyn Round>;
    println!("{}", c);
    println!("The area is {}", c.area());
}

trait Shape {
    fn area(self: Box<Self>) -> f64;
}

trait Round {
    fn get_radius(&self) -> f64;
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Round for Circle {
    fn get_radius(&self) -> f64 {
        self.radius
    }
}

impl Shape for dyn Round {
    fn area(self: Box<Self>) -> f64 {
        std::f64::consts::PI * self.get_radius() * self.get_radius()
    }
}

impl Display for dyn Round {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{{ radius:{} }}", self.get_radius())
    }
}
