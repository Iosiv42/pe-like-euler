fn main() {
    let mut digits: Vec<usize> = vec![0; (1000. * 2.0_f32.log10()).ceil() as usize];
    digits[0] = 2;
    let mut carry = 0;
    for _ in 1..1000 {
        for idx in 0..digits.len() {
            let digit = digits[idx];
            let prod = digit * 2;
            let da = carry + prod;
            digits[idx] = da % 10;
            carry = da / 10;
        }
    }

    println!("{}", digits.into_iter().sum::<usize>());
}
