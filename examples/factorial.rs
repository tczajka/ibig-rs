use ibig::prelude::*;

fn main() {
    let n: u32 = 10000;
    let mut factorial = ubig!(1);
    for i in 1..=n {
        factorial *= UBig::from(i);
    }
    println!("{}! = {}", n, factorial);
}
