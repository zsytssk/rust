fn main() {
    let t = "4ttt";

    let parse = t.parse::<i32>();
    match parse {
        Ok(tt) => println!("sucess => {:?}", tt),
        Err(tt) => println!("err => {:?}", tt),
    }
}
