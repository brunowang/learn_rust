fn main() {
    let p = (1, 3);
    let func = add2;
    println!("evaluation output {}", func(p));

    let func = add1 as fn((i32, i32)) -> i32;
    println!("evaluation output {}", func(p));
    let mut func: fn((i32, i32)) -> i32 = add1;
    println!("evaluation output {}", func(p));
    func = add2;
    println!("evaluation output {}", func(p));
    // func = _add3;
    test_inner();

    const DIM: usize = cube(2);
    const ARR: [i32; DIM] = [0; DIM];
    println!("{:?}", ARR);

    let fib8 = fibonacci(8);
    println!("{}", fib8);

    for arg in std::env::args() {
        println!("Arg: {}", arg);
        // var()函数可以接受一个字符串类型参数，用于查找当前环境变量中是否存在这个名字的环境变量，vars()函数不携带参数，可以返回所有的环境变量。
        match std::env::var(&arg) {
            Ok(val) => println!("{}: {:?}", &arg, val),
            Err(e) => println!("couldn't find environment {}, {}", &arg, e),
        }
        println!(
            "All environment variable count {}",
            std::env::vars().count()
        );
    }

    std::process::exit(0);
}

fn add1(t: (i32, i32)) -> i32 {
    t.0 + t.1
}

fn add2((x, y): (i32, i32)) -> i32 {
    x + y
}

fn _add3(x: i32, y: i32) -> i32 {
    x + y
}

fn test_inner() {
    static INNER_STATIC: i64 = 42;

    fn internal_incr(x: i64) -> i64 {
        x + 1
    }

    struct InnerTemp(i64);

    impl InnerTemp {
        fn incr(&mut self) {
            self.0 = internal_incr(self.0);
        }
    }

    let mut t = InnerTemp(INNER_STATIC);
    t.incr();
    println!("{}", t.0);
}

fn _diverges() -> ! {
    panic!("This function never returns!");
}

const fn cube(num: usize) -> usize {
    num * num * num
}

fn fibonacci(index: u32) -> u64 {
    if index == 1 || index == 2 {
        1
    } else {
        fibonacci(index - 1) + fibonacci(index - 2)
    }
}
