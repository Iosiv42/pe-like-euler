use primes::PrimeSet;

fn right_truncates(num: usize) -> Vec<usize> {
    let mut truncates = Vec::new();
    let mut num_cpy = num;
    while num_cpy > 0 {
        truncates.push(num_cpy);
        num_cpy /= 10;
    }

    truncates
}

fn left_truncates(num: usize) -> Vec<usize> {
    let digits_count = (num as f32).log10().floor() as u32 + 1;
    let mut k = digits_count - 1;
    let mut truncates = Vec::with_capacity(digits_count as usize);
    let mut num_cpy = num;

    while num_cpy > 0 {
        truncates.push(num_cpy);
        num_cpy %= 10_usize.pow(k);
        k = k.checked_sub(1).unwrap_or_default();
    }

    truncates
}

fn main() {
    let mut ans = 0;
    for prime in primes::Sieve::new().iter() {
        if prime > 1_000_000 {
            break;
        }
        if prime < 10 {
            continue;
        }

        let prime = prime as usize;
        if right_truncates(prime).into_iter().all(|n| primes::is_prime(n as u64))
            && left_truncates(prime).into_iter().all(|n| primes::is_prime(n as u64)) {
            ans += prime;
        }
    }

    println!("{}", ans);
}
