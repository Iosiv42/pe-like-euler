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
    // It can be roughly calculated from state that PrimePi(n) ~ {n/ln(n)}.
    // Used LambertW to find inverse of n/ln(n) then find that value.
    // Also knowing that PrimePi(n) > {n/ln(n)} we guarantee that there'll be
    // the answer.
    let upper_bound = 116_684;
    println!("{}", sieve(upper_bound)[10_001 - 1]);
}
