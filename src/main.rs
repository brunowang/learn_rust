use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum Sex {
    Male,
    Female,
}

struct User {
    name: String,
    age: u8,
    sex: Sex,
    desc: Option<String>,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Err(e) = write!(
            f,
            "Name: {}, Age: {}, Sex: {:?}",
            self.name, self.age, self.sex,
        ) {
            return Err(e);
        }
        if let Some(s) = &self.desc {
            write!(f, ", Desc: {}", s)
        } else {
            Ok(())
        }
    }
}

fn main() {
    let user: User = User {
        name: String::from("brunowang"),
        age: 30,
        sex: Sex::Male,
        desc: Some(String::from("Administrator")),
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

    let my: (&str, u8, Sex, Option<String>) = (&user.name, user.age, user.sex, user.desc);
    println!("{:#?}, {}, {}, {:?}, {:?}", my, my.0, my.1, my.2, my.3);
}
