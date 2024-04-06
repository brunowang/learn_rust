use std::fmt::{Display, Formatter};

struct User {
    name: String,
    age: u8,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}, Age: {}", self.name, self.age)
    }
}

fn main() {
    let user: User = User {
        name: String::from("brunowang"),
        age: 30,
    };
    println!("Hello, world! {}", user);

    let tags: [&str; 3] = ["java", "php", "js"];
    for i in 0..tags.len() {
        println!("range: {:#?}", tags[i])
    }
    for item in tags.iter() {
        println!("iterator: {:#?}", item)
    }
    for (idx, item) in tags.iter().enumerate() {
        println!("index: {}, item: {:#?}", idx, item)
    }
    let mut tags: [u8; 10] = [0; 10];
    for i in 0..tags.len() {
        tags[i] = (i + 1) as u8;
    }
    println!("{:?}", tags);

    let my: (&str, u8) = ("brunowang", 30);
    println!("{:#?}, {}, {}", my, my.0, my.1);
}
