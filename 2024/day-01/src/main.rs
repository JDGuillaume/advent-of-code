use std::{env::current_dir, fs::File, io::Read};

use regex::Regex;

fn read_file_to_string(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}

fn parse_lists(buffer: &str) -> (Vec<u32>, Vec<u32>) {
    let re = Regex::new(r"\d+").unwrap();
    let results: Vec<u32> = re
        .find_iter(buffer)
        .map(|m| m.as_str().parse().unwrap())
        .collect();

    let mut vec_1: Vec<u32> = Vec::new();
    let mut vec_2: Vec<u32> = Vec::new();

    for (index, result) in results.iter().enumerate() {
        match index % 2 {
            0 => vec_1.push(*result),
            _ => vec_2.push(*result),
        }
    }

    vec_1.sort();
    vec_2.sort();

    (vec_1, vec_2)
}

fn calculate_distance(list_1: &[u32], list_2: &[u32]) -> u32 {
    let mut distance = 0;

    for (index, _location) in list_1.iter().enumerate() {
        let difference = list_1[index].abs_diff(list_2[index]);
        distance += difference;
    }

    distance
}

fn calculate_similarity(list_1: &[u32], list_2: &[u32]) -> u32 {
    let mut similarity = 0;

    for location in list_1 {
        let matches: Vec<&u32> = list_2.iter().filter(|x| **x == *location).collect();
        similarity += location * matches.len() as u32;
    }

    similarity
}

fn main() {
    let path = format!("{}/locations.txt", current_dir().unwrap().display());
    let buffer = read_file_to_string(&path);

    let (vec_1, vec_2) = parse_lists(&buffer);

    println!(
        "The total distance observed between the two lists is: {}.",
        calculate_distance(&vec_1, &vec_2)
    );

    println!(
        "The observed similarity between the two lists is: {}.",
        calculate_similarity(&vec_1, &vec_2)
    );
}
