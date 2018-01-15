fn is_odd(n: u32) -> bool {
    n % 2 == 1
}
fn is_not_odd(n: u32) -> bool {
    n % 2 == 0
}
fn is_odd3(n: u32) -> bool {
    n % 3 == 0
}
fn main() {
    let upper = 1000;
    let mut acc = 0;

    for n in 0.. {
        let n_squared = n * n;

        if n_squared >= upper {
            break;
        } else if is_not_odd(n_squared) {
            continue;
        } else if is_odd3(n_squared) {
            continue;
        }
        acc += n_squared;
    }

    println!("{}", acc);

    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n_squared| n_squared < upper)
        .filter(|&n_squared| !is_odd3(n_squared))
        .filter(|&n_squared| is_odd(n_squared))
        .fold(0, |acc, n_squared| acc + n_squared);
    println!("{}", sum_of_squared_odd_numbers);
}
