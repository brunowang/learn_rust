use std::fmt::{Display, Formatter, Error};

fn main() {
    let c = Circle{radius: 2f64};
    println!("{:?}", c);
    println!("{:#?}", c);
    let c = Box::new(c) as Box<dyn Round>;
    println!("{}", c);
    println!("The area is {}", c.area());
    let x : i32 = 10.double();
    println!("{}", x);
    let me = Chef;
    <dyn Cook>::start(&me);
    <Chef as Wash>::start(&me);

    check_type(T::get1);
    check_type(T::get2);
    check_type(get3);
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

trait Double {
    fn double(&self) -> Self;
}

impl Double for i32 {
    fn double(&self) -> i32 {
        *self * 2
    }
}

// Fully Qualified Syntax. <T as TraitName>::item
trait Cook {
    fn start(&self);
}

trait Wash {
    fn start(&self);
}

struct Chef;

impl Cook for Chef {
    fn start(&self) {
        println!("Cook::start");
    }
}

impl Wash for Chef {
    fn start(&self) {
        println!("Wash::start");
    }
}

// There is no difference between methods and functions
struct T(usize);

impl T {
    fn get1(&self) -> usize {self.0}
    fn get2(&self) -> usize {self.0}
}

fn get3(t: &T) -> usize {t.0}

fn check_type(_: fn(&T) -> usize) {}
