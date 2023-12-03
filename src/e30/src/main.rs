fn digitize(num: usize, base: usize) -> Vec<usize> {
    if num == 0 {
        return vec![0];
    }

    let mut digits = Vec::new();
    let mut num_cpy = num;
    while num_cpy > 0 {
        digits.push(num_cpy % base);
        num_cpy /= base;
    }

    return digits;
}

fn main() {
    let mut ans = 0;
    for num in 10..1_000_000 {
        let da = digitize(num, 10).into_iter()
            .map(|d| d.pow(5))
            .sum::<usize>();
        if da == num {
            ans += num;
        }
    }

    println!("{ans}");
}
