fn digitize(n: usize, base: usize) -> Vec<u8> {
    if n == 0 {
        return vec![0];
    }

    let mut n_cpy = n;
    let mut d: u8;
    let mut digits: Vec<u8> = vec![];
    while n_cpy != 0 {
        d = (n_cpy % base) as u8;
        n_cpy = n_cpy / 10;
        digits.push(d);
    }
    return digits;
}

fn is_pal(n: usize) -> bool {
    let digs = digitize(n, 10);
    return digs.iter().eq(digs.iter().rev());
}

fn main() {
    let mut max_pal = 0;
    for a in (100..=999).rev() {
        for b in (a..=999).rev() {
            let prod = a*b;
            if prod <= max_pal {
                break;
            }
            if is_pal(prod) {
                max_pal = prod;
            }
        }
    }

    println!("{max_pal}");
}
