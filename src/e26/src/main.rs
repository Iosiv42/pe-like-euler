use num_bigint::ToBigUint;

fn main() {
    // To get more involved in problem read thread on math stackexchange,
    // because of my file margins to small to fit the explanation.
    // Here it is: https://math.stackexchange.com/questions/377683/length-of-period-of-decimal-expansion-of-a-fraction

    let mut ans = 0;
    let mut max_len = 0;
    let big_ten = 10.to_biguint().unwrap();
    for d in 2_usize..1000_usize {
        if d % 2 == 0 || d % 5 == 0 {
            continue;
        }

        let mut cycle_len = 1;
        while &big_ten.pow(cycle_len) % &d != 1.to_biguint().unwrap() {
            cycle_len += 1;
        }

        if cycle_len > max_len {
            max_len = cycle_len;
            ans = d;
        }
    }

    println!("{}", ans);
}
