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
    let factorial = |n: usize| (2..=n).product::<usize>();

    let mut ans = 0;
    for num in 3..=(6 * factorial(9)) {
        if digitize(num, 10).into_iter().map(factorial).sum::<usize>() == num {
            ans += num;
        }
    }

    println!("{ans}")
}
