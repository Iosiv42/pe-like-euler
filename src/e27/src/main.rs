fn is_prime(num: isize) -> bool {
    if num < 2 {
        return false; 
    }
    if num == 2 || num == 3 {
        return true;
    }

    for d in 2..(num as f32).sqrt().ceil() as usize {
        if num % d as isize == 0 {
            return false;
        }
    }

    return true;
}

fn main() {
    let mut max_seq_len = 0;
    let mut max_prod = 0;

    for a in -999..1000 {
        for b in -1000..=1000 {
            let poly = |n: isize| n.pow(2) + a*n + b;

            for (seq_len, n) in (0..).enumerate() {
                if !is_prime(poly(n)) {
                    if seq_len > max_seq_len {
                        max_seq_len = seq_len;
                        max_prod = a*b;
                    }
                    break;
                }
            }
        }
    }

    println!("{}", max_prod);
}
