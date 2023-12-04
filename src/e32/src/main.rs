use std::collections::HashSet;

fn is_pandigital(num_str: String) -> bool {
    let mut digit_count = vec![0; 9];
    for digit in num_str.chars() {
        let idig = digit.to_digit(10).unwrap();
        if idig < 1 {
            return false;
        }

        let idx = digit.to_digit(10).unwrap() as usize - 1;
        digit_count[idx] += 1;
    }

    digit_count.iter().all(|&ele| ele == 1)
}

fn main() {
    // I weigh the problem and decide that straightforward method would be
    // better than generating all permutations and placing muitiply and equal
    // sign at all positions.

    let mut pandigital_prods = HashSet::new();
    for m1 in 1..=2000 {
        for m2 in m1..=2000 {
            let prod = m1 * m2;

            let mut cand_str = String::with_capacity(9);
            for n in [m1, m2, prod] {
                cand_str.push_str(&n.to_string());
            }

            if is_pandigital(cand_str) {
                println!("{} * {} = {}", m1, m2, prod);
                pandigital_prods.insert(prod);
            }
        }
    }

    println!("{}", pandigital_prods.into_iter().sum::<isize>());
}
