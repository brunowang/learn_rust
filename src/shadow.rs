fn main() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let v = v;
    for i in &v {
        println!("{}", i)
    }

    let v = Vec::new();
    let mut v = v;
    v.push(1);
    println!("{:?}", v);
}
