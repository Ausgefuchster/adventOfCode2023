use std::{env, fs};

fn main() {
    let file = env::args().nth(1).expect("Please supply a file name");
    let input = fs::read_to_string(file).expect("Cannot find file");

    println!("Solution 1: {}", solution_one(&input));
    println!("Solution 2: {}", solution_two(&input));
}

struct BoatRace {
    time: u32,
    distance: u32,
}

fn solution_one(input: &str) -> usize {
    let times: Vec<u32> = input
        .lines()
        .nth(0)
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let distances: Vec<u32> = input
        .lines()
        .nth(1)
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let races: Vec<BoatRace> = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| BoatRace {
            time: *t,
            distance: *d,
        })
        .collect();

    races
        .iter()
        .map(|race| {
            (1..race.time)
                .filter(|i| (i * (race.time - i)) > race.distance)
                .count()
        })
        .reduce(|a, b| a * b)
        .unwrap_or(0)
}

fn solution_two(input: &str) -> usize {
    let time: u64 = input
        .lines()
        .nth(0)
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap();

    let distance: u64 = input
        .lines()
        .nth(1)
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap();

    (1..time)
        .filter(|i| (i * (time - i)) > distance)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one() {
        let input = fs::read_to_string("example.txt").expect("Cannot find file");
        assert_eq!(solution_one(&input), 288);
    }

    #[test]
    fn two() {
        let input = fs::read_to_string("example.txt").expect("Cannot find file");
        assert_eq!(solution_two(&input), 71503);
    }
}
