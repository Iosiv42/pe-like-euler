use std::collections::HashSet;

fn divisors(num: usize) -> HashSet<usize> {
    let mut divs = HashSet::new();
    for dividend in 1..=((num as f32).sqrt().ceil() as usize) {
        if num % dividend == 0 {
            divs.insert(dividend);
            divs.insert(num / dividend);
        }
    }

    return divs;
}

fn main() {
    let d = |num: usize| divisors(num).into_iter().sum::<usize>() - num;
    let mut amicables = HashSet::new();
    for num in 1..10_000 {
        if amicables.contains(&num) {
            continue;
        }

        let b = d(num);
        if d(b) == num && num != b {
            amicables.insert(num);
            amicables.insert(b);
        }
    }

    println!("{}", amicables.into_iter().sum::<usize>());
}
