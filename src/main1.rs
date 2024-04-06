use crate::models::user::{new_user_info, new_user_score, UserInfo, UserScore};

mod models;

fn main() {
    println!("Hello, world! from main1");

    let mut user: UserInfo = new_user_info(123, String::from("brunowang"), 19);
    let user: &UserInfo = user
        .add_user_tag("java")
        .add_user_tag("php")
        .add_user_tag("js")
        .add_user_tag("go")
        .add_user_tag("rust");

    println!("{:?}", user);
    println!("{}, {}, {}, {:?}", user.id, user.name, user.age, user.tags);

    let mut score: UserScore<i32, i32> = new_user_score(101, 90);
    let score: &UserScore<_, _> = score.set_comment("A类用户");
    println!(
        "{:?}, {}, {}",
        score,
        score.get_score(),
        score.get_comment()
    );
    let mut score: UserScore<&str, f32> = new_user_score("K01", 90.0);
    let score: &UserScore<_, _> = score.set_comment("B类用户");
    println!(
        "{:?}, {}, {}",
        score,
        score.get_score(),
        score.get_comment()
    );
}
