use std::{env, fs};

#[derive(Debug)]
struct Mapping {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

#[derive(Debug)]
struct Input {
    seeds: Vec<u64>,
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

fn main() {
    let file = env::args().nth(1).expect("Please supply a file name");
    let input = fs::read_to_string(file).expect("Cannot find file");

    println!("Solution 1: {}", solution_one(&input));
}

fn parse_mappings(paragraph: &str) -> Vec<Mapping> {
    let lines: Vec<&str> = paragraph.lines().collect();
    let lines = &lines[1..];

    lines
        .iter()
        .map(|line| {
            let split: Vec<u64> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            Mapping {
                destination_range_start: split[0],
                source_range_start: split[1],
                range_length: split[2],
            }
        })
        .collect()
}

fn parse_input(raw: &str) -> Input {
    let paragraphs: Vec<&str> = raw.split("\r\n\r\n").collect();

    Input {
        seeds: paragraphs[0]["seeds: ".len()..]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect(),
        seed_to_soil: parse_mappings(paragraphs[1]),
        soil_to_fertilizer: parse_mappings(paragraphs[2]),
        fertilizer_to_water: parse_mappings(paragraphs[3]),
        water_to_light: parse_mappings(paragraphs[4]),
        light_to_temperature: parse_mappings(paragraphs[5]),
        temperature_to_humidity: parse_mappings(paragraphs[6]),
        humidity_to_location: parse_mappings(paragraphs[7]),
    }
}

fn solution_one(input: &str) -> usize {
    let input = parse_input(input);
    let soils: Vec<u64> = resolve_mapping(input.seeds, input.seed_to_soil);
    let fertilizers: Vec<u64> = resolve_mapping(soils, input.soil_to_fertilizer);
    let waters: Vec<u64> = resolve_mapping(fertilizers, input.fertilizer_to_water);
    let lights: Vec<u64> = resolve_mapping(waters, input.water_to_light);
    let temperatures: Vec<u64> = resolve_mapping(lights, input.light_to_temperature);
    let humidities: Vec<u64> = resolve_mapping(temperatures, input.temperature_to_humidity);
    let locations: Vec<u64> = resolve_mapping(humidities, input.humidity_to_location);

    *locations.iter().min().unwrap() as usize
}

fn resolve_mapping(sources: Vec<u64>, mappings: Vec<Mapping>) -> Vec<u64> {
    sources
        .iter()
        .map(|source| source_to_destination(*source, &mappings))
        .collect()
}

fn source_to_destination(source: u64, mappings: &[Mapping]) -> u64 {
    mappings
        .iter()
        .find(|mapping| is_source_in_range(source, mapping))
        .map(|mapping| mapping.destination_range_start + (source - mapping.source_range_start))
        .unwrap_or(source)
}

fn is_source_in_range(source: u64, mapping: &Mapping) -> bool {
    mapping.source_range_start <= source
        && source < mapping.source_range_start + mapping.range_length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = fs::read_to_string("example.txt").expect("Cannot find file");
        assert_eq!(solution_one(&input), 35);
    }
}
