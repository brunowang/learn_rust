#[derive(Debug, Clone, Copy)]
struct Num(i32);

fn main() {
    let v: Vec<Num> = vec![Num(1), Num(2), Num(3), Num(4), Num(5)];
    {
        let center = v[2];
        println!("{:?}", center);
        // drop(center);
    }
    println!("{:?}", v);
}
