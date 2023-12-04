use std::collections::HashSet;
use num::integer::gcd;

fn digitize(num: usize, base: usize) -> Vec<usize> {
    if num == 0 {
        return vec![0];
    }

    let mut num_cpy = num;
    let mut digits = Vec::new();
    while num_cpy > 0 {
        let digit = num_cpy % base;
        num_cpy /= base;
        digits.push(digit);
    }

    digits
}

fn digits_to_num(digits: &Vec<usize>) -> usize {
    digits.iter()
        .enumerate()
        .map(|(n, &d)| 10_usize.pow(n as u32) * d)
        .sum::<usize>()
}

fn cancel_same(
    numer: &Vec<usize>,
    denumer: &Vec<usize>
) -> (Vec<usize>, Vec<usize>) {
    let mut new_numer = numer.clone();
    let mut new_denumer = denumer.clone();

    new_numer.retain(|d| !denumer.contains(d) || *d == 0);
    new_denumer.retain(|d| !numer.contains(d) || *d == 0);

    (new_numer, new_denumer)
}

fn main() {
    // Pretty wierd problem.

    let mut prod_numer = 1;
    let mut prod_denumer = 1;

    for denumer in 2..100 {
        for numer in 1..denumer {
            let nd = digitize(numer, 10);
            let dd = digitize(denumer, 10);
            let (new_nd, new_dd) = cancel_same(&nd, &dd);

            let frac = numer as f32 / denumer as f32;
            let new_frac = digits_to_num(&new_nd) as f32 / digits_to_num(&new_dd) as f32;

            if nd != new_nd && dd != new_dd && frac == new_frac {
                prod_denumer *= denumer;
                prod_numer *= numer;
            }
        }
    }

    println!("{}", prod_denumer / gcd(prod_numer, prod_denumer));
}
