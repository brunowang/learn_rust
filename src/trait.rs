fn main() {
    let c = Box::new(Circle{radius: 2f64}) as Box<dyn Round>;
    println!("The area is {}", c.area());
}

trait Shape {
    fn area(self: Box<Self>) -> f64;
}

trait Round {
    fn get_radius(&self) -> f64;
}

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
