fn main() {
    // 我们需要这个动态数组本身是“可变的”,才能获得它的“可变借用指针”
    let mut v = vec![];
    // 在函数调用的时候,同时也要显示获取它的“可变借用指针”
    vec_push(&mut v);
    // 打印结果,可以看到v已经被改变
    println!("{:?}", v);

    let mut var = 0_i32;
    {
        let p1 = &mut var; // p1 指针本身不能被重新绑定,我们可以通过p1改变变量var的值
        *p1 = 1;
    }
    {
        let temp = 2_i32;
        let mut p2 = &var; // 我们不能通过p2改变变量var的值,但p2指针本身指向的位置可以被改变
        p2 = &temp;
    }
    {
        let mut temp = 3_i32;
        let mut p3 = &mut var; // 我们既可以通过p3改变变量var的值,而且p3指针本身指向的位置也可以被改变
        *p3 = 3;
        p3 = &mut temp;
    }
}

fn vec_push(v: &mut Vec<i32>) {
    v.push(5);
}
