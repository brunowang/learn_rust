fn main() {
    let x = Some(Some(5));
    println!("{:?}", deep_match(x));

    let y = Some(Some(100));
    println!("{:?}", deep_match(y));
}

fn deep_match(v: Option<Option<i32>>) -> Option<i32> {
    match v {
        Some(r @ Some(1..=10)) => r,
        _ => None,
    }
}
