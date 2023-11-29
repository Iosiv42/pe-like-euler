fn score(idx: usize, word: String) -> usize {
    let alpha_val: usize = word.chars().map(|ch| ch as usize - 64).sum();
    return alpha_val * idx;
}

fn main() {
    let mut words: Vec<String> = std::fs::read_to_string("names.txt").unwrap()
        .replace("\"", "")
        .split(",")
        .map(|e| e.to_owned())
        .collect();
    words.sort();

    let ans: usize = words.iter()
        .enumerate()
        .map(|(idx, word)| score(idx + 1, word.to_owned()))
        .sum();
    println!("{ans}");
}
