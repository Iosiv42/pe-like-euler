use std::thread::panicking;

use permutator::CartesianProductIterator;

fn bin_to_int(digits: &Vec<usize>) -> usize {
    digits.iter().rev()
        .enumerate()
        .map(|(n, d)| d * 2_usize.pow(n as u32))
        .sum()
}

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

fn is_palindrome(num: usize) -> bool {
    let digits = digitize(num, 10);
    digits.iter().eq(digits.iter().rev())
}

fn main() {
    let data = [[0, 1].as_slice(); 10];
    let iter = CartesianProductIterator::new(data.as_slice());

    let mut ans = 0;
    for left_side in iter {
        let da: Vec<usize> = left_side.iter()
            .skip_while(|&&&d| d == 0)
            .map(|e| **e)
            .collect();

        for yi in [vec![], vec![0], vec![1]] {
            if yi.len() > 0 && da.len() == 10 {
                continue;
            }

            let pal: Vec<usize> = da.iter()
                .chain(yi.iter())
                .chain(da.iter().rev())
                .map(|e| *e)
                .collect();

            if pal.len() > 0 && is_palindrome(bin_to_int(&pal)) {
                ans += bin_to_int(&pal);
            }
        }
    }

    println!("{}", ans)
}
