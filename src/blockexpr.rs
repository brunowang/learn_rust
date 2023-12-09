fn main() {
    let x : () = {println!("Hello."); };
    let y : i32 = {println!("Hello."); 10 };
    let z : i32 = if false { 20 } else { 30 };
    println!("{:?} {} {} {} {} {}", x, y, z, my_func(), my_loop(), my_loop_tag());
}

fn my_func() -> i32 {
    100
}

fn my_loop() -> u32 {
    let mut count = 0u32;
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough\n");
            break count;
        }
    }
}

fn my_loop_tag() -> i32 {
    let mut m = 1;
    let n = 10;
    'a: loop {
        if m < 100 {
            m += 1;
        } else {
            'b: loop {
                if m + n > 50 {
                    println!("break tag a");
                    break 'a m+n;
                } else {
                    continue 'a;
                }
            }
        }
    }
}
