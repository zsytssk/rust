fn main() {
    // let to_string = Box::new(|x: &i32| -> String { x.to_string() });
    let list_of_numbers = vec![1, 2, 3];
    let list_of_string: Vec<String> = list_of_numbers.iter().map(&(*return_closure())).collect();
    // let list_of_string: Vec<String> = list_of_numbers.iter().map(*to_string).collect();

    println!("{:?}", list_of_string);
}

fn return_closure() -> Box<Fn(&i32) -> String> {
    Box::new(|x| x.to_string())
}
