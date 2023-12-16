use std::{env, fs};

fn main() {
    let file = env::args().nth(1).expect("Please supply a file name");
    let input = fs::read_to_string(file).expect("Cannot find file");

    println!("Solution 1: {}", solution_one(&input));
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = fs::read_to_string("example.txt").expect("Cannot find file");
        assert_eq!(solution_one(&input), 288);
    }
}
