use std::collections::HashMap;

fn main() {
    let mut map = HashMap::<String, String>::new();
    let key = String::from("abc");
    process_or_default(&mut map, key);
}

fn process_or_default(map: &mut HashMap<String, String>, key: String)
{
    match map.get_mut(&key) {
        Some(value) => println!("{}", value),
        None => {
            map.insert(key, String::new());
        }
    }
}
