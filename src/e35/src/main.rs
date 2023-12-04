use primes::PrimeSet;

fn all_rotations(num: usize) -> Vec<usize> {
    let mut rotations = vec![num];

    let k = (num as f32).log10() as u32;
    let mut num_cpy = num;
    for _ in 0..k {
        num_cpy = num_cpy/10 + 10_usize.pow(k) * (num_cpy % 10);
        rotations.push(num_cpy);
    }

    rotations
}

fn main() {
    let mut ans = 0;
    for prime in primes::Sieve::new().iter() {
        if prime >= 1_000_000 {
            break;
        }

        if all_rotations(prime as usize)
            .into_iter()
            .all(|n| primes::is_prime(n as u64))
        {
            ans += 1;
        }
    }

    println!("{}", ans);
}
