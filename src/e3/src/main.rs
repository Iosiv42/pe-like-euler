use std::cmp::max;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }

    for dividend in 2..(n as f32).sqrt() as u64 {
        if n % dividend == 0 {
            return false;
        }
    }

    return true;
}


fn main() {
    let num: u64 = 600851475143;
    let mut max_prime = 1;

    for dividend in 2..=(num as f32).sqrt() as u64 {
        if num % dividend == 0 {
            if is_prime(num / dividend) {
                max_prime = max(max_prime, num / dividend);
            } else if is_prime(dividend) {
                max_prime = max(max_prime, dividend);
            }
        }
    }
    println!("{}", max_prime);
}
