#[macro_use]
mod macro_rules;

fn main() {
    let counts = hashmap!['A' => 0, 'C' => 0, 'G' => 0, 'T' => 0];
    println!("{:?}", counts);

    echo!();
    echo!("abc");
    echo!("ab", "bc", "cd");
}
