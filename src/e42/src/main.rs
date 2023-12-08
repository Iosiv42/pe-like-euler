/// Inverse of triangle numbers function.
fn inverse_triangle(t: f32) -> f32 {
    -0.5 + 0.5 * (8. * t + 1.).sqrt()
}

fn main() {
    let data = std::fs::read_to_string("src/words.txt")
        .expect("File 'words.txt' didn't found.")
        .replace("\"", "");
    let words: Vec<_> = data
        .split(",")
        .collect();

    let word_value = |word: &str| {
        word.chars().map(|ch| ch as usize - 64).sum::<usize>()
    };
    let ans = words.iter()
        .map(|&word| word_value(word))
        .filter(|&cand| inverse_triangle(cand as f32) % 1. == 0.)
        .count();

    println!("{}", ans);
}
