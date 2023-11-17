//! Uh, kinda different approach. I think it might be way to go,
//! but it's funny. R_SQRT_5 is reciprocal of sqrt(5).
//! 33 fib. num is last fib < 4*10^6. All even fibs. go with
//! step 3. It's obvious, because odd + odd = even and odd + even = odd.

const PHI: f64 = 1.61803398874989484820458683436563;
const R_SQRT_5: f64 = 0.44721359549995793928183473374625;

fn main() {
    println!(
        "{}",
        (1..=11)
            .map(|k| (PHI.powi(3*k) * R_SQRT_5).round() as isize)
            .sum::<isize>()
    );
}
