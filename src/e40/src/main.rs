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

fn main() {
    let mut digits = Vec::with_capacity(1_000_000);
    for n in 1.. {
        if digits.len() >= 1_000_000 {
            break;
        }

        digits.extend(digitize(n, 10).into_iter().rev());
    }

    let ans: usize = (0..=6)
        .map(|k| digits[10_usize.pow(k) - 1])
        .product();
    println!("{}", ans);
}
