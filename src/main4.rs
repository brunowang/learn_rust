use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f: File = File::open("src/test.txt").expect("file error");
    let mut buf: String = String::new();
    f.read_to_string(&mut buf).expect("read error");
    println!("{}", buf);
}
