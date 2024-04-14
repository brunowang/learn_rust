#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

impl Clone for User {
    fn clone(&self) -> Self {
        User {
            id: self.id,
            name: self.name.clone(),
            age: self.age,
        }
    }
}

fn main() {
    let a = User {
        id: 123,
        name: String::from("Bruno"),
        age: 33,
    };
    let b = a.clone();
    println!("a.id={}, b.id={}", a.id, b.id);
    println!("a={:?}, b={:?}", a, b);
}
