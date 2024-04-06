use std::num::Wrapping;

fn main() {
    let variable = 0x_1234_ABCD;
    println!("{}", variable);
    let variable = 123usize;
    println!("{}", variable);
    let variable = 0x_ff_u8;
    println!("{}", variable);
    let i = 100_i8;
    println!("checked {:?}", i.checked_add(i));
    println!("saturating {:?}", i.saturating_add(i));
    println!("wrapping {:?}", i.wrapping_add(i));

    let big = Wrapping(std::u32::MAX);
    let sum = big + Wrapping(2_u32);
    println!("{}", sum.0);

    println!("{}", 12E+4_f64);

    let mut small = std::f32::EPSILON;
    while small > 0. {
        small /= 2.0;
        println!("{} {:?}", small, small.classify());
    }

    let x = 1.0f32 / 0.0;
    let y = 0.0f32 / 0.0;
    println!("{} {}", x, y);

    let inf = std::f32::INFINITY;
    println!("{} {} {}", inf * 0.0, 1.0 / inf, inf / inf);
}
