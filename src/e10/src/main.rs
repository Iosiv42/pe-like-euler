fn sieve(upper: usize) -> Vec<usize> {
    let upper = upper + 1;
    let mut is_primes: Vec<usize> = (0..upper).collect();
    for n in 2..=(upper as f32).sqrt().ceil() as usize {
        if is_primes[n] != 0 {
            for idx in (n*n..upper).step_by(n) {
                is_primes[idx] = 0;
            }
        }
    }
    
    return is_primes.into_iter().filter(|&n| n > 1).collect();
}

fn main() {
    println!("{}", sieve(2_000_000).iter().sum::<usize>());
}
