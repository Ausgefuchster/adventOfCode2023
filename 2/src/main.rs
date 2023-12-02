use std::{env, fs};

const ALLOWED_RED: u32 = 12;
const ALLOWED_GREEN: u32 = 13;
const ALLOWED_BLUE: u32 = 14;

struct Row {
    game_id: u32,
    cube_entries: Vec<CubeEntry>,
}

struct CubeEntry {
    red: u32,
    green: u32,
    blue: u32,
}

trait Power {
    fn power(&self) -> u32;
}

impl Power for Vec<CubeEntry> {
    fn power(&self) -> u32 {
        let max_red = self.iter().map(|entry| entry.red).max().unwrap_or(0);
        let max_green = self.iter().map(|entry| entry.green).max().unwrap_or(0);
        let max_blue = self.iter().map(|entry| entry.blue).max().unwrap_or(0);

        max_red * max_green * max_blue
    }
}

fn main() {
    let file = env::args().nth(1).expect("Please supply a file name");
    let input = fs::read_to_string(file).expect("Cannot find file");

    let sum: u32 = input
        .lines()
        .map(build_row)
        .filter(all_cube_entries_valid)
        .map(|row| row.game_id)
        .sum();

    println!("First Sum: {}", sum);

    let sum: u32 = input
        .lines()
        .map(build_row)
        .map(|row| row.cube_entries.power())
        .sum();

    println!("Second Sum: {}", sum);
}

fn build_row(row: &str) -> Row {
    let game_id: u32 = get_game_id(row);
    let cube_entries: Vec<CubeEntry> = row
        .split(':')
        .nth(1)
        .expect("No cube entries")
        .split(';')
        .map(parse_cube_entry)
        .collect();
    Row {
        game_id,
        cube_entries,
    }
}

fn all_cube_entries_valid(row: &Row) -> bool {
    row.cube_entries.iter().all(|entry| {
        entry.red <= ALLOWED_RED && entry.green <= ALLOWED_GREEN && entry.blue <= ALLOWED_BLUE
    })
}

fn parse_cube_entry(entry: &str) -> CubeEntry {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    entry.split(',').for_each(|c| match c {
        x if x.ends_with("red") => {
            red = x
                .replace("red", "")
                .trim()
                .parse()
                .expect("Cannot parse red");
        }
        x if x.ends_with("green") => {
            green = x
                .replace("green", "")
                .trim()
                .parse()
                .expect("Cannot parse green");
        }
        x if x.ends_with("blue") => {
            blue = x
                .replace("blue", "")
                .trim()
                .parse()
                .expect("Cannot parse blue");
        }
        _ => {}
    });

    CubeEntry { red, green, blue }
}

fn get_game_id(row: &str) -> u32 {
    row.split(':')
        .next()
        .expect("No game id")
        .replace("Game", "")
        .trim()
        .parse()
        .expect("Cannot parse game id")
}
