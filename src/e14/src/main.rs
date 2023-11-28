fn collatz_seq_len(num: usize) -> usize {
    let mut num = num;
    let mut len = 1;
    while num != 1 {
        if num % 2 == 0 {
            num /= 2;
        } else {
            num = 3*num + 1;
        }

        len += 1
    }

    return len;
}

fn main() {
    let max = (1..1_000_000)
        .map(|num| collatz_seq_len(num))
        .enumerate()
        .max_by_key(|pair| pair.1)
        .unwrap();
    println!("{}", max.0 + 1);
}
