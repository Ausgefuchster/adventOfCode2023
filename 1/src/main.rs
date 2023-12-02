use std::{env, fs};

enum NumberAsString {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

fn main() {
    let file = env::args().nth(1).expect("Please supply a file name");
    let input = fs::read_to_string(file).expect("Cannot find file");

    let sum: u32 = part_one(input.as_str());

    println!("Sum: {}", sum);
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let first = s
                .chars()
                .find(|c| c.is_ascii_digit())
                .expect("First digit must exist");
         
            let last = s
                .chars()
                .rev()
                .find(|c| c.is_ascii_digit())
                .unwrap_or(first);

            let mut result = String::new();
            result.push(first);
            result.push(last);

            result.parse::<u32>().expect("Cannot parse")
        })
        .sum()
}
