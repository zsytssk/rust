use std::ops::Add;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
enum Inch {}

#[derive(Debug, Clone, Copy)]
enum Mm {}

#[derive(Debug, Clone, Copy)]
struct Length(f64);

impl Add for Length {
    type Output = Length;
    fn add(self, rhs: Length) -> Length {
        Length(self.0 + rhs.0)
    }
}

fn main() {
    let one_foot: Length = Length(12.0);
    let one_meter: Length = Length(1000.0);

    let two_feet = one_foot + one_meter;
    let two_meter = one_meter + one_meter;
    println!("{:?}", two_feet.0);
    println!("{:?}", two_meter.0);
}
