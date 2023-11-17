fn main() {
    // Sum of arithmetic progression.
    let ap_sum = |start: isize, end: isize, step: isize| {
        (start + end) * ((end - start) / step + 1) / 2
    };

    println!(
        "{}",
        ap_sum(3, 999, 3) + ap_sum(5, 995, 5) - ap_sum(15, 990, 15)
    );
}
