fn main() {
    // Merely remember about that F_n ~ phi^n / sqrt(5).
    // and to get amount of digits in number "n" you need to calculate
    // log10(n) + 1. Therefore, one just has to solve that kinda easy
    // equation. log10(F_n) + 1 = 1000 and then round it.

    let phi: f64 = (1. + 5.0_f64.sqrt()) / 2.;
    let approx = (999. + 0.5*5.0_f64.log10()) / phi.log10();
    println!("{}", approx.round() as usize)
}
