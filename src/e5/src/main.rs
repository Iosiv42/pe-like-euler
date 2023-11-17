//! Is it problem for schoolkids?
extern crate num;
use num::integer::lcm;

fn main() {
    let ans = (1..=20).reduce(|acc, n| lcm(acc, n)).unwrap();
    println!("{}", ans);
}
