fn times2(value: i32) -> i32 {
    2*value
}

fn fun_test(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn main() {
    let a = fun_test(100, times2);
    println!("{}", a);

}