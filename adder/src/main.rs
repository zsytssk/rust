extern crate add_one;
extern crate add_two;
extern crate rand;

fn main() {
    let num = 10;
    println!("{}", add_one::add_one(num));
    println!("{}", add_two::add_two(num));
}
