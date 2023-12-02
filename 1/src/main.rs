use std::{env, fs};

enum NumberAsString {
    Zero(String),
    One(String),
    Two(String),
    Three(String),
    Four(String),
    Five(String),
    Six(String),
    Seven(String),
    Eight(String),
    Nine(String),
}

impl NumberAsString {
    fn from_str(s: &str) -> Option<NumberAsString> {
        match s {
            "zero" => Some(NumberAsString::Zero("0".to_string())),
            "one" => Some(NumberAsString::One("1".to_string())),
            "two" => Some(NumberAsString::Two("2".to_string())),
            "three" => Some(NumberAsString::Three("3".to_string())),
            "four" => Some(NumberAsString::Four("4".to_string())),
            "five" => Some(NumberAsString::Five("5".to_string())),
            "six" => Some(NumberAsString::Six("6".to_string())),
            "seven" => Some(NumberAsString::Seven("7".to_string())),
            "eight" => Some(NumberAsString::Eight("8".to_string())),
            "nine" => Some(NumberAsString::Nine("9".to_string())),
            _ => None,
        }
    }

    fn as_string(&self) -> String {
        match self {
            NumberAsString::Zero(s) => s.to_string(),
            NumberAsString::One(s) => s.to_string(),
            NumberAsString::Two(s) => s.to_string(),
            NumberAsString::Three(s) => s.to_string(),
            NumberAsString::Four(s) => s.to_string(),
            NumberAsString::Five(s) => s.to_string(),
            NumberAsString::Six(s) => s.to_string(),
            NumberAsString::Seven(s) => s.to_string(),
            NumberAsString::Eight(s) => s.to_string(),
            NumberAsString::Nine(s) => s.to_string(),
        }
    }
}

fn main() {
    let file = env::args().nth(1).expect("Please supply a file name");
    let input = fs::read_to_string(file).expect("Cannot find file");

    let sum: u32 = part_one(input.as_str());
    println!("First Sum: {}", sum);

    let sum: u32 = part_two(input.as_str());
    println!("Second Sum: {}", sum);
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

fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let first = s
                .chars()
                .enumerate()
                .find_map(|(idx, c)| {
                    if c.is_ascii_digit() {
                        return Some((String::from(c), idx));
                    }
                    for i in 3..6 {
                        if idx + i >= s.len() {
                            continue;
                        }
                        if let Some(n) = NumberAsString::from_str(&s[idx..idx + i]) {
                            return Some((n.as_string(), idx + i));
                        }
                    }
                    None
                })
                .expect("First digit must exist");

            let last = (first.1..s.len())
                .rev()
                .find_map(|idx| {
                    if s.chars().nth(idx).unwrap().is_ascii_digit() {
                        return Some(String::from(s.chars().nth(idx).unwrap()));
                    }
                    for i in 3..6 {
                        let index = idx + 1;
                        if index.checked_sub(i).is_none() {
                            continue;
                        }
                        if let Some(n) = NumberAsString::from_str(&s[index - i..index]) {
                            return Some(n.as_string());
                        }
                    }
                    None
                })
                .unwrap_or(first.0.to_string());

            let mut result = String::new();
            result.push_str(&first.0);
            result.push_str(&last);

            result.parse::<u32>().expect("Cannot parse")
        })
        .sum()
}
