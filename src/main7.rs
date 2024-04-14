#[derive(Debug, Copy, Clone)]
struct User {
    id: i32
}

fn main() {
    let a = User{id: 123};
    let b = a;
    println!("a.id={}, b.id={}", a.id, b.id);
    println!("a={:?}, b={:?}", a, b);
}
