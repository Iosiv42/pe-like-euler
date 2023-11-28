fn main() {
    let factorial = |n: usize| (1..=n as u128).product::<u128>();
    println!("{}", (21..=40).product::<u128>() / factorial(20));
}
