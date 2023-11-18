fn sigma(n: usize, k: u32) -> usize {
    if n == 0 {
        panic!("Don't do it. 0 has infinitly many factors.")
    }

    let mut result = 0;
    for dividend in 2..=(n as f32).sqrt().ceil() as usize {
        if n % dividend == 0 {
            result += dividend.pow(k) + (n / dividend).pow(k);
        }
    }

    return result;
}

fn main() {
    let (mut n, mut i) = (1, 2);
    while sigma(n, 0) < 500 {
        n += i;
        i += 1;
    }

    println!("{n}");
}
