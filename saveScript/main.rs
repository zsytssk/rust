extern crate rand;

use rand::random;
use std::time::Instant;

fn main() {
    let x = random::<u16>();
    let start = Instant::now();
    println!("start:{}", x);

    fn matchRandom(mut n: i32, x: u16, start: Instant) {
        for i in 1..n {
            let y = random::<u16>();
            if (y == x) {
                let elapsed = start.elapsed();
                let t = (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64;
                println!("{}:{}:{}", y, n, t);
                return;
            }
        }
        n = n * 10;
        matchRandom(n * 10, x, start);
    }
    matchRandom(10, x, start);
}
