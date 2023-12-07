use std::process::exit;

use itertools::Itertools;

fn to_num(digits: &Vec<usize>) -> usize {
    digits.iter()
        .rev()
        .enumerate()
        .map(|(n, d)| 10_usize.pow(n as u32) * d)
        .sum()
}

fn main() {
    // We starts from 7-digit pandigitals because of sum of digits of 8- and 9-digit
    // pandigitals is always divisible by 3, therefore isn't prime.
    for n in (1..=7).rev() {
        for perm in (1..=n).rev().permutations(n) {
            if primes::is_prime(to_num(&perm) as u64) {
                println!("{}", to_num(&perm));
                exit(0);
            }
        }
    }
}
