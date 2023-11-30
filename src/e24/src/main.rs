use itertools::Itertools;

fn main() {
    // As a simple solutionm I leave it here, but as more robust approach
    // you can think about  that after 9! iterations, first digit will be
    // changed to next in alpabetical order. So construct so many iters
    // such that n*9! will be < 10e6. Then you've 10e6 - n*9! iters that
    // needed to be performed to reach the answer. Repeat it for sorted set
    // of digits that you have but without that stands at first place.
    // But now permutation will be length of 9. Deep into recursion to
    // find the lurking answer.
    
    for (no, perm) in (0..=9).permutations(10).enumerate() {
        if no == 999_999 {
            println!("{:?}", perm);
            break;
        }
    }
}
