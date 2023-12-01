fn main() {
    let mut curr_num = 3;
    let mut step = 2;
    let mut n = 0;
    let mut diag_sum = 1;

    while curr_num <= 1001_usize.pow(2) {
        diag_sum += curr_num;

        step += 2 * (n == 3) as usize;
        curr_num += step;

        n += 1;
        n %= 4;
    }

    println!("{diag_sum}");
}
