const ONES: [&'static str; 10] = [
    "",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

const SPECIAL_TENS: [&'static str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&'static str; 10] = [
    "",
    "",
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
];

const HUNDREDS: [&'static str; 2] = [
    "",
    "hundred"
];

fn nth_digit(num: usize, nth: u32) -> usize {
    return num % 10_usize.pow(nth) / 10_usize.pow(nth - 1);
}

fn num_to_sentence(num: usize) -> String {
    let mut sentence: Vec<String> = vec![];

    if (num % 100 >= 10)
        && (num % 100 < 20)
    {
        sentence.push(SPECIAL_TENS[nth_digit(num, 1)].to_owned());
    } else {
        sentence.push(ONES[nth_digit(num, 1)].to_owned());
        sentence.push(TENS[nth_digit(num, 2)].to_owned())
    }

    if num % 100 != 0 && num > 100 {
        sentence.push("and".to_owned());
    }

    sentence.push(HUNDREDS[(nth_digit(num, 3) >= 1) as usize].to_owned());
    sentence.push(ONES[nth_digit(num, 3)].to_owned());

    sentence.reverse();
    return sentence.join("");
}

fn main() {
    let mut ans = "onethousand".len();
    ans += (1..1000)
        .map(|num| num_to_sentence(num).len())
        .sum::<usize>();
    println!("{}", ans);
}
